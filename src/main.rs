// Composition Root: apenas monta as camadas e sobe o servidor.
use std::net::SocketAddr;

use poc_cadastro_transportadora_rust::criar_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1..3) Infraestrutura, injeção de dependências e camada HTTP já compostas
    //       em `criar_app` (compartilhado com os testes de integração).
    let app = criar_app("sqlite:transportadoras.db?mode=rwc").await?;

    // 4) Servidor.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Servidor rodando em http://{addr}");
    println!("Swagger UI:   http://{addr}/swagger-ui/");
    println!("OpenAPI JSON: http://{addr}/openapi.json");

    axum::serve(listener, app).await?;

    Ok(())
}
