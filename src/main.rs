use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Transportadora {
    id: String,
    nome: String,
    cnpj: String,
    email: String,
    telefone: String,
    endereco: String,
    ativo: bool,
}

#[derive(Debug, Deserialize)]
struct CriarTransportadora {
    nome: String,
    cnpj: String,
    email: String,
    telefone: String,
    endereco: String,
}

async fn listar_transportadoras(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Transportadora>>, (StatusCode, String)> {
    let lista = sqlx::query_as::<_, Transportadora>(
        "SELECT id, nome, cnpj, email, telefone, endereco, ativo FROM transportadoras ORDER BY nome",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(lista))
}

async fn criar_transportadora(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CriarTransportadora>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO transportadoras (id, nome, cnpj, email, telefone, endereco, ativo) \
         VALUES (?, ?, ?, ?, ?, ?, 1)",
    )
    .bind(&id)
    .bind(&payload.nome)
    .bind(&payload.cnpj)
    .bind(&payload.email)
    .bind(&payload.telefone)
    .bind(&payload.endereco)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let criada = Transportadora {
        id,
        nome: payload.nome,
        cnpj: payload.cnpj,
        email: payload.email,
        telefone: payload.telefone,
        endereco: payload.endereco,
        ativo: true,
    };

    Ok((StatusCode::CREATED, Json(criada)))
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:transportadoras.db?mode=rwc")
        .await?;

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
    .execute(&pool)
    .await?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/transportadoras", get(listar_transportadoras))
        .route("/transportadoras", post(criar_transportadora))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}