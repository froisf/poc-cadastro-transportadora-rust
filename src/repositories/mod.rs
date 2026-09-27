// Camada de infraestrutura: abstrações de acesso a dados.
pub mod sqlite_transportadora_repository;

use async_trait::async_trait;

use crate::{error::AppResult, models::Transportadora};

/// Interface (port) do repositório — equivalente ao `TransportadoraRepository` do Spring Data.
///
/// O Service depende **apenas** desta trait (Inversão de Dependência),
/// permitindo trocar SQLite por Postgres/Mongo sem alterar regras de negócio.
#[async_trait]
pub trait TransportadoraRepository: Send + Sync {
    /// Lista todas as transportadoras.
    async fn listar(&self) -> AppResult<Vec<Transportadora>>;

    /// Busca uma transportadora pelo id. Retorna `None` se não existir.
    async fn buscar_por_id(&self, id: &str) -> AppResult<Option<Transportadora>>;

    /// Persiste uma nova transportadora e devolve a entidade criada.
    async fn criar(&self, transportadora: &Transportadora) -> AppResult<Transportadora>;

    /// Substitui os dados da transportadora. Retorna `None` se o id não existir.
    async fn atualizar(
        &self,
        id: &str,
        transportadora: &Transportadora,
    ) -> AppResult<Option<Transportadora>>;

    /// Remove a transportadora. Retorna `false` se o id não existir.
    async fn deletar(&self, id: &str) -> AppResult<bool>;
}
