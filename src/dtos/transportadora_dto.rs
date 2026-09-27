use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Corpo da requisição `POST /transportadoras`.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[schema(example = json!({
    "nome": "Transportes Rapidos Ltda",
    "cnpj": "11.222.333/0001-81",
    "email": "contato@rapidos.com.br",
    "telefone": "1133334444",
    "endereco": "Rua das Flores, 100 - Sao Paulo/SP",
}))]
pub struct CriarTransportadora {
    /// Razão social ou nome fantasia (2 a 120 caracteres).
    #[schema(min_length = 2, max_length = 120, example = "Transportes Rapidos Ltda")]
    pub nome: String,

    /// CNPJ com ou sem máscara. Os dígitos verificadores são validados e a
    /// máscara é removida na gravação (persistido apenas com dígitos).
    #[schema(pattern = r"^[\d./\- ]{14,18}$", example = "11.222.333/0001-81")]
    pub cnpj: String,

    /// E-mail de contato no formato `local@dominio.com`.
    #[schema(example = "contato@rapidos.com.br")]
    pub email: String,

    /// Telefone com DDD (10 dígitos) ou celular (11 dígitos).
    #[schema(pattern = r"^\d{10,11}$", example = "1133334444")]
    pub telefone: String,

    /// Endereço completo, não vazio.
    #[schema(example = "Rua das Flores, 100 - Sao Paulo/SP")]
    pub endereco: String,
}

/// Corpo da requisição `PUT /transportadoras/{id}` (substituição total).
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[schema(example = json!({
    "nome": "Transportes Rapidos Ltda",
    "cnpj": "11.222.333/0001-81",
    "email": "novo-contato@rapidos.com.br",
    "telefone": "11987654321",
    "endereco": "Av. Paulista, 1000 - Sao Paulo/SP",
}))]
pub struct AtualizarTransportadora {
    /// Razão social ou nome fantasia (2 a 120 caracteres).
    #[schema(min_length = 2, max_length = 120, example = "Transportes Rapidos Ltda")]
    pub nome: String,

    /// CNPJ com ou sem máscara. Os dígitos verificadores são validados e a
    /// máscara é removida na gravação (persistido apenas com dígitos).
    #[schema(pattern = r"^[\d./\- ]{14,18}$", example = "11.222.333/0001-81")]
    pub cnpj: String,

    /// E-mail de contato no formato `local@dominio.com`.
    #[schema(example = "contato@rapidos.com.br")]
    pub email: String,

    /// Telefone com DDD (10 dígitos) ou celular (11 dígitos).
    #[schema(pattern = r"^\d{10,11}$", example = "11987654321")]
    pub telefone: String,

    /// Endereço completo, não vazio.
    #[schema(example = "Av. Paulista, 1000 - Sao Paulo/SP")]
    pub endereco: String,
}

/// Corpo padrão das respostas de erro da aplicação.
///
/// `campo` identifica qual campo da requisição falhou na validação, permitindo
/// ao consumidor corrigir apenas o campo problemático. Vazio quando o erro não
/// tem origem em um campo específico (404, 409, 500).
#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "status": 400,
    "erro": "formato inválido",
    "campo": "email",
}))]
pub struct ErroResponse {
    /// Código HTTP correspondente ao erro.
    #[schema(example = 400)]
    pub status: u16,

    /// Mensagem legível do erro.
    #[schema(example = "formato inválido")]
    pub erro: String,

    /// Campo da requisição que originou o erro, quando houver.
    #[schema(example = "email", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campo: Option<String>,
}
