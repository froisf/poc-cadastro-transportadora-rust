use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::dtos::transportadora_dto::ErroResponse;

/// Erros do domínio traduzidos para HTTP (equivalente ao `@ControllerAdvice` do Spring).
///
/// Implementar `IntoResponse` centraliza o tratamento de erro e permite que
/// qualquer camada retorne `Result<_, AppError>`.
#[derive(Debug, Error)]
pub enum AppError {
    /// 404 - recurso inexistente.
    #[error("{0}")]
    NaoEncontrado(String),

    /// 400 - falha de validação de um campo.
    #[error("campo '{campo}' inválido: {mensagem}")]
    Validacao { campo: String, mensagem: String },

    /// 409 - conflito de unicidade (ex.: CNPJ já cadastrado).
    #[error("{0}")]
    Conflito(String),

    /// 500 - falha técnica do banco (convertida automaticamente de `sqlx::Error`).
    #[error("falha ao acessar o banco de dados: {0}")]
    Banco(#[from] sqlx::Error),
}

impl AppError {
    pub fn nao_encontrado(recurso: &str) -> Self {
        Self::NaoEncontrado(format!("{recurso} não encontrado(a)"))
    }

    pub fn validacao(campo: &str, mensagem: &str) -> Self {
        Self::Validacao {
            campo: campo.to_string(),
            mensagem: mensagem.to_string(),
        }
    }

    pub fn conflito(mensagem: &str) -> Self {
        Self::Conflito(mensagem.to_string())
    }
}

/// Mapeia cada variante de erro para o respectivo status HTTP.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensagem, campo) = match self {
            AppError::NaoEncontrado(m) => (StatusCode::NOT_FOUND, m, None),
            AppError::Validacao { campo, mensagem } => {
                (StatusCode::BAD_REQUEST, mensagem, Some(campo))
            }
            AppError::Conflito(m) => (StatusCode::CONFLICT, m, None),
            // O detalhe técnico fica no log; o cliente recebe uma mensagem genérica.
            AppError::Banco(e) => {
                eprintln!("[erro] falha de banco de dados: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "erro interno ao acessar os dados".to_string(),
                    None,
                )
            }
        };

        (
            status,
            Json(ErroResponse {
                status: status.as_u16(),
                erro: mensagem,
                campo,
            }),
        )
            .into_response()
    }
}

/// Atalho para o `Result` padrão de qualquer camada da aplicação.
pub type AppResult<T> = Result<T, AppError>;
