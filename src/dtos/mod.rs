// Camada de contrato: DTOs de entrada/saída da API (não vazam para o domínio).
pub mod transportadora_dto;

pub use transportadora_dto::{AtualizarTransportadora, CriarTransportadora, ErroResponse};
