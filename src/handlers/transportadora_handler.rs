use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dtos::{AtualizarTransportadora, CriarTransportadora, ErroResponse},
    error::AppResult,
    handlers::AppState,
    models::Transportadora,
};

// Handlers: apenas traduzem HTTP <-> caso de uso. Nenhuma regra de negócio aqui.

/// Documentação OpenAPI da API de transportadoras.
///
/// A apresentação (`/swagger-ui`) é montada em `lib.rs`, que também expõe este
/// mesmo documento em `/openapi.json`.
#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "API de Cadastro de Transportadoras",
        version = "0.1.0",
        description = "CRUD de transportadoras com validação de CNPJ (dígitos verificadores) \
                      e unicidade de CNPJ garantida por índice único no banco."
    ),
    paths(
        health,
        listar_transportadoras,
        buscar_transportadora,
        criar_transportadora,
        atualizar_transportadora,
        deletar_transportadora,
    ),
    components(schemas(Transportadora, CriarTransportadora, AtualizarTransportadora, ErroResponse)),
    tags(
        (name = "health", description = "Verificação de disponibilidade"),
        (name = "transportadoras", description = "CRUD de transportadoras"),
    )
)]
pub struct ApiDoc;

/// GET /health
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Servidor no ar", content_type = "text/plain",
         body = String, example = "ok"),
    )
)]
pub async fn health() -> &'static str {
    "ok"
}

/// GET /transportadoras
#[utoipa::path(
    get,
    path = "/transportadoras",
    tag = "transportadoras",
    responses(
        (status = 200, description = "Lista de transportadoras, ordenada por nome",
         body = Vec<Transportadora>),
        (status = 500, description = "Erro interno do banco",
         body = ErroResponse, content_type = "application/json"),
    )
)]
pub async fn listar_transportadoras(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Transportadora>>> {
    let transportadoras = state.transportadora_service.listar().await?;
    Ok(Json(transportadoras))
}

/// GET /transportadoras/{id} -> 200 | 404
#[utoipa::path(
    get,
    path = "/transportadoras/{id}",
    tag = "transportadoras",
    params(("id" = String, Path, description = "Identificador (UUID) da transportadora")),
    responses(
        (status = 200, description = "Transportadora encontrada", body = Transportadora),
        (status = 404, description = "Transportadora não encontrada",
         body = ErroResponse, content_type = "application/json"),
        (status = 500, description = "Erro interno do banco",
         body = ErroResponse, content_type = "application/json"),
    )
)]
pub async fn buscar_transportadora(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Transportadora>> {
    let transportadora = state.transportadora_service.buscar_por_id(&id).await?;
    Ok(Json(transportadora))
}

/// POST /transportadoras -> 201 | 400 | 409
#[utoipa::path(
    post,
    path = "/transportadoras",
    tag = "transportadoras",
    request_body = CriarTransportadora,
    responses(
        (status = 201, description = "Transportadora criada com sucesso", body = Transportadora),
        (status = 400, description = "Campo inválido (o campo é sinalizado em `campo`)",
         body = ErroResponse, content_type = "application/json"),
        (status = 409, description = "CNPJ já cadastrado",
         body = ErroResponse, content_type = "application/json"),
        (status = 500, description = "Erro interno do banco",
         body = ErroResponse, content_type = "application/json"),
    )
)]
pub async fn criar_transportadora(
    State(state): State<AppState>,
    Json(dto): Json<CriarTransportadora>,
) -> AppResult<(StatusCode, Json<Transportadora>)> {
    let transportadora = state.transportadora_service.criar(dto).await?;
    Ok((StatusCode::CREATED, Json(transportadora)))
}

/// PUT /transportadoras/{id} -> 200 | 400 | 404
#[utoipa::path(
    put,
    path = "/transportadoras/{id}",
    tag = "transportadoras",
    params(("id" = String, Path, description = "Identificador (UUID) da transportadora")),
    request_body = AtualizarTransportadora,
    responses(
        (status = 200, description = "Transportadora atualizada", body = Transportadora),
        (status = 400, description = "Campo inválido (o campo é sinalizado em `campo`)",
         body = ErroResponse, content_type = "application/json"),
        (status = 404, description = "Transportadora não encontrada",
         body = ErroResponse, content_type = "application/json"),
        (status = 409, description = "CNPJ já cadastrado em outra transportadora",
         body = ErroResponse, content_type = "application/json"),
        (status = 500, description = "Erro interno do banco",
         body = ErroResponse, content_type = "application/json"),
    )
)]
pub async fn atualizar_transportadora(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(dto): Json<AtualizarTransportadora>,
) -> AppResult<Json<Transportadora>> {
    let transportadora = state.transportadora_service.atualizar(&id, dto).await?;
    Ok(Json(transportadora))
}

/// DELETE /transportadoras/{id} -> 204 | 404
#[utoipa::path(
    delete,
    path = "/transportadoras/{id}",
    tag = "transportadoras",
    params(("id" = String, Path, description = "Identificador (UUID) da transportadora")),
    responses(
        (status = 204, description = "Transportadora excluída (sem corpo de resposta)"),
        (status = 404, description = "Transportadora não encontrada",
         body = ErroResponse, content_type = "application/json"),
        (status = 500, description = "Erro interno do banco",
         body = ErroResponse, content_type = "application/json"),
    )
)]
pub async fn deletar_transportadora(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    state.transportadora_service.deletar(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
