//! Biblioteca do POC de cadastro de transportadoras.
//!
//! Existe como alvo de biblioteca para que os testes de integração em `tests/`
//! consigam montar a aplicação e exercitá-la pelas rotas HTTP reais.
//! O `main.rs` continua sendo o composition root do executável.

pub mod dtos;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;

pub use handlers::transportadora_handler::ApiDoc;

use axum::Router;
use error::AppResult;
use handlers::{AppState, router};
use repositories::sqlite_transportadora_repository::SqliteTransportadoraRepository;
use services::TransportadoraService;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

/// Injeta as camadas a partir de um pool já conectado e monta as rotas.
fn montar_app(pool: SqlitePool) -> AppResult<Router> {
    let repository = SqliteTransportadoraRepository::new(pool);
    let service = TransportadoraService::new(repository);

    Ok(router(AppState::new(service)))
}

/// Composição da aplicação: pool -> repository -> service -> router.
///
/// Compartilhada entre o binário e os testes de integração, garantindo que
/// ambos exercitem exatamente a mesma montagem de camadas.
///
/// `url_banco` deve apontar para um SQLite em modo RW. Em memória
/// (ex.: `sqlite::memory:`) use `max_connections = 1`, pois cada conexão
/// abriria um banco distinto.
pub async fn criar_app(url_banco: &str) -> AppResult<Router> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url_banco)
        .await?;
    SqliteTransportadoraRepository::inicializar_schema(&pool).await?;

    montar_app(pool)
}

/// Monta a aplicação sobre um banco SQLite em memória, isolado por chamada.
///
/// Usado pelos testes de integração para não depender do `transportadoras.db`
/// da raiz do projeto. `max_connections = 1` é obrigatório: com mais de uma
/// conexão, o SQLite criaria um banco em memória diferente por conexão.
pub async fn criar_app_em_memoria() -> AppResult<Router> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    SqliteTransportadoraRepository::inicializar_schema(&pool).await?;

    montar_app(pool)
}
