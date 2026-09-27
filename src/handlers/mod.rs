// Camada de entrega (HTTP): converte requisição em chamada de caso de uso.
pub mod transportadora_handler;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    repositories::sqlite_transportadora_repository::SqliteTransportadoraRepository,
    services::TransportadoraService,
};

/// Estado compartilhado da aplicação (equivalente ao container de beans do Spring).
///
/// É `Clone` porque o Axum clona o estado a cada requisição; o pool de conexões
/// é referenciado por `Arc` internamente, então a clonagem é barata.
#[derive(Debug, Clone)]
pub struct AppState {
    pub transportadora_service: TransportadoraService<SqliteTransportadoraRepository>,
}

impl AppState {
    pub fn new(
        transportadora_service: TransportadoraService<SqliteTransportadoraRepository>,
    ) -> Self {
        Self {
            transportadora_service,
        }
    }
}

/// Declara as rotas da API.
/// No Axum 0.8 o parâmetro de path é declarado com `{}` (ex.: `/transportadoras/{id}`).
///
/// Também publica a documentação OpenAPI:
/// - `/openapi.json`  -> documento OpenAPI 3.1 em JSON
/// - `/swagger-ui`    -> interface interativa (redireciona para `/swagger-ui/`)
pub fn router(state: AppState) -> Router {
    use axum::routing::get;
    use transportadora_handler::{
        atualizar_transportadora, buscar_transportadora, criar_transportadora,
        deletar_transportadora, health, listar_transportadoras, ApiDoc,
    };

    // `merge` combina as rotas da API com as do Swagger UI sem sobrescrever
    // os handlers existentes.
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .url("/openapi.json", ApiDoc::openapi());

    Router::new()
        .route("/health", get(health))
        .route(
            "/transportadoras",
            get(listar_transportadoras).post(criar_transportadora),
        )
        .route(
            "/transportadoras/{id}",
            get(buscar_transportadora)
                .put(atualizar_transportadora)
                .delete(deletar_transportadora),
        )
        .merge(swagger_ui)
        .with_state(state)
}
