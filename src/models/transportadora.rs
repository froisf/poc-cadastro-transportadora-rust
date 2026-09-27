use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Entidade de domínio `Transportadora`.
///
/// - `FromRow` permite o mapeamento automático da linha do SQLite para a entidade.
/// - `Serialize` expõe a entidade na resposta HTTP.
/// - `Clone` é exigida para que o Service (e o estado do Axum) sejam compartilháveis.
/// - `ToSchema` publica a entidade no OpenAPI sem acoplar o domínio ao utoipa
///   além da macro (o `cnpj` é persistido só com dígitos, apesar de a entrada
///   aceitar a máscara).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[schema(example = json!({
    "id": "9f8c1f7e-2b4a-4d3e-8a1b-0c5d6e7f8a9b",
    "nome": "Transportes Rapidos Ltda",
    "cnpj": "11222333000181",
    "email": "contato@rapidos.com.br",
    "telefone": "1133334444",
    "endereco": "Rua das Flores, 100 - Sao Paulo/SP",
    "ativo": true,
}))]
pub struct Transportadora {
    /// Identificador único (UUID v4) gerado no servidor.
    #[schema(example = "9f8c1f7e-2b4a-4d3e-8a1b-0c5d6e7f8a9b")]
    pub id: String,

    /// Razão social ou nome fantasia da transportadora.
    #[schema(min_length = 2, max_length = 120, example = "Transportes Rapidos Ltda")]
    pub nome: String,

    /// CNPJ apenas com dígitos: a máscara é removida na gravação.
    #[schema(pattern = r"^\d{14}$", example = "11222333000181")]
    pub cnpj: String,

    /// E-mail de contato, sempre em minúsculas.
    #[schema(example = "contato@rapidos.com.br")]
    pub email: String,

    /// Telefone com DDD, de 10 a 11 dígitos.
    #[schema(pattern = r"^\d{10,11}$", example = "1133334444")]
    pub telefone: String,

    /// Endereço completo da transportadora.
    #[schema(example = "Rua das Flores, 100 - Sao Paulo/SP")]
    pub endereco: String,

    /// Indica registro ativo. Sempre `true` em operações via API.
    pub ativo: bool,
}

impl Transportadora {
    /// Cria uma nova transportadora já com `id` gerado (regra de negócio do domínio).
    pub fn nova(
        nome: String,
        cnpj: String,
        email: String,
        telefone: String,
        endereco: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            nome,
            cnpj,
            email,
            telefone,
            endereco,
            ativo: true,
        }
    }
}
