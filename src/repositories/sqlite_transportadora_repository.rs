use async_trait::async_trait;
use sqlx::{Row, SqlitePool, sqlite::SqliteRow};

use crate::{
    error::{AppError, AppResult},
    models::Transportadora,
    repositories::TransportadoraRepository,
};

/// SQL 100% estático: no sqlx 0.9 apenas `&'static str` é aceito sem `AssertSqlSafe`.
const SELECT_TODAS: &str = "SELECT id, nome, cnpj, email, telefone, endereco, ativo
                           FROM transportadoras
                           ORDER BY nome";

const SELECT_POR_ID: &str = "SELECT id, nome, cnpj, email, telefone, endereco, ativo
                             FROM transportadoras
                             WHERE id = ?";

/// Implementação concreta da porta `TransportadoraRepository` sobre SQLite.
///
/// Única camada que conhece SQL. O `SqlitePool` é `Clone`, então o repositório
/// (e por consequência o Service) é cheaply clonável para o estado do Axum.
#[derive(Debug, Clone)]
pub struct SqliteTransportadoraRepository {
    pool: SqlitePool,
}

impl SqliteTransportadoraRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Cria a estrutura da tabela de forma idempotente (garante compatibilidade
    /// com o `transportadoras.db` já existente na raiz do projeto).
    pub async fn inicializar_schema(pool: &SqlitePool) -> AppResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS transportadoras (
                id TEXT PRIMARY KEY,
                nome TEXT NOT NULL,
                cnpj TEXT NOT NULL,
                email TEXT NOT NULL,
                telefone TEXT NOT NULL,
                endereco TEXT NOT NULL,
                ativo BOOLEAN NOT NULL DEFAULT 1
            )",
        )
        .execute(pool)
        .await?;

        // Índice único de CNPJ: garante unicidade na própria base (regra de integridade).
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS ux_transportadoras_cnpj ON transportadoras(cnpj)",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Converte violação de unicidade do SQLite em `AppError::Conflito` (HTTP 409).
    fn mapear_erro_sqlite(erro: sqlx::Error) -> AppError {
        let eh_unico = erro
            .as_database_error()
            .is_some_and(|db| db.is_unique_violation());

        if eh_unico {
            AppError::conflito("já existe uma transportadora com este CNPJ")
        } else {
            AppError::from(erro)
        }
    }

    /// Mapeia a linha do banco para a entidade de domínio.
    fn mapear_linha(row: &SqliteRow) -> AppResult<Transportadora> {
        Ok(Transportadora {
            id: row.try_get("id")?,
            nome: row.try_get("nome")?,
            cnpj: row.try_get("cnpj")?,
            email: row.try_get("email")?,
            telefone: row.try_get("telefone")?,
            endereco: row.try_get("endereco")?,
            ativo: row.try_get("ativo")?,
        })
    }
}

#[async_trait]
impl TransportadoraRepository for SqliteTransportadoraRepository {
    async fn listar(&self) -> AppResult<Vec<Transportadora>> {
        let rows = sqlx::query(SELECT_TODAS)
            .fetch_all(&self.pool)
            .await
            .map_err(Self::mapear_erro_sqlite)?;

        rows.iter().map(Self::mapear_linha).collect()
    }

    async fn buscar_por_id(&self, id: &str) -> AppResult<Option<Transportadora>> {
        let row = sqlx::query(SELECT_POR_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::mapear_erro_sqlite)?;

        row.as_ref().map(Self::mapear_linha).transpose()
    }

    async fn criar(&self, transportadora: &Transportadora) -> AppResult<Transportadora> {
        sqlx::query(
            "INSERT INTO transportadoras (id, nome, cnpj, email, telefone, endereco, ativo)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&transportadora.id)
        .bind(&transportadora.nome)
        .bind(&transportadora.cnpj)
        .bind(&transportadora.email)
        .bind(&transportadora.telefone)
        .bind(&transportadora.endereco)
        .bind(transportadora.ativo)
        .execute(&self.pool)
        .await
        .map_err(Self::mapear_erro_sqlite)?;

        Ok(transportadora.clone())
    }

    async fn atualizar(
        &self,
        id: &str,
        transportadora: &Transportadora,
    ) -> AppResult<Option<Transportadora>> {
        let resultado = sqlx::query(
            "UPDATE transportadoras
             SET nome = ?, cnpj = ?, email = ?, telefone = ?, endereco = ?, ativo = ?
             WHERE id = ?",
        )
        .bind(&transportadora.nome)
        .bind(&transportadora.cnpj)
        .bind(&transportadora.email)
        .bind(&transportadora.telefone)
        .bind(&transportadora.endereco)
        .bind(transportadora.ativo)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(Self::mapear_erro_sqlite)?;

        if resultado.rows_affected() == 0 {
            return Ok(None);
        }

        Ok(Some(Transportadora {
            id: id.to_string(),
            nome: transportadora.nome.clone(),
            cnpj: transportadora.cnpj.clone(),
            email: transportadora.email.clone(),
            telefone: transportadora.telefone.clone(),
            endereco: transportadora.endereco.clone(),
            ativo: transportadora.ativo,
        }))
    }

    async fn deletar(&self, id: &str) -> AppResult<bool> {
        let resultado = sqlx::query("DELETE FROM transportadoras WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Self::mapear_erro_sqlite)?;

        Ok(resultado.rows_affected() > 0)
    }
}
