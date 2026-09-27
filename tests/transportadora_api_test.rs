//! Testes de integração da API de transportadoras.
//!
//! Exercitam a aplicação completa (rotas -> handlers -> service -> repository ->
//! SQLite) em um banco **em memória**, criado por teste. Nenhum arquivo do
//! projeto (`transportadoras.db`) é tocado e os testes são independentes.

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode, header::CONTENT_TYPE},
};
use poc_cadastro_transportadora_rust::criar_app_em_memoria;
use serde_json::{Value, json};
use tower::ServiceExt;

/// CNPJ válido (dígitos verificadores conferem) usado como base dos cenários.
const CNPJ_VALIDO: &str = "11.222.333/0001-81";

/// Payload mínimo válido para `POST /transportadoras`.
fn payload_valido() -> Value {
    json!({
        "nome": "Transportes Rapidos Ltda",
        "cnpj": CNPJ_VALIDO,
        "email": "contato@rapidos.com.br",
        "telefone": "1133334444",
        "endereco": "Rua das Flores, 100 - Sao Paulo/SP",
    })
}

/// Monta uma requisição JSON a partir de um `Value`.
fn requisicao_json(
    metodo: Method,
    uri: &str,
    corpo: &Value,
) -> Request<Body> {
    Request::builder()
        .method(metodo)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(corpo).expect("payload serializável")))
        .expect("requisição válida")
}

/// Monta uma requisição com corpo JSON cru (para testar JSON malformado).
fn requisicao_json_cru(
    metodo: Method,
    uri: &str,
    corpo: &str,
) -> Request<Body> {
    Request::builder()
        .method(metodo)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(corpo.to_string()))
        .expect("requisição válida")
}

/// Envia a requisição ao router e devolve `(status, corpo)`.
async fn enviar(app: &Router, requisicao: Request<Body>) -> (StatusCode, Vec<u8>) {
    let resposta = app
        .clone()
        .oneshot(requisicao)
        .await
        .expect("router não deveria falhar");

    let status = resposta.status();
    let corpo = to_bytes(resposta.into_body(), 1024 * 1024)
        .await
        .expect("corpo da resposta legível")
        .to_vec();

    (status, corpo)
}

/// Deserializa o corpo de erro em `serde_json::Value` (o `ErroResponse` do
/// projeto só é `Serialize`, então usamos `Value` para inspecioná-lo).
fn corpo_json(corpo: &[u8]) -> Value {
    serde_json::from_slice(corpo)
        .unwrap_or_else(|e| panic!("corpo não é JSON válido ({e}): {corpo:?}"))
}

// ---------------------------------------------------------------------------
// Cenário 1: criação com sucesso -> 201
// ---------------------------------------------------------------------------

#[tokio::test]
async fn cria_transportadora_com_sucesso_retorna_201() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    let (status, corpo) = enviar(
        &app,
        requisicao_json(
            Method::POST,
            "/transportadoras",
            &payload_valido(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "corpo: {corpo:?}");

    let criada = corpo_json(&corpo);
    assert!(!criada["id"].as_str().unwrap().is_empty());
    assert_eq!(criada["nome"], "Transportes Rapidos Ltda");
    // O service normaliza o CNPJ (máscara removida) e o e-mail (minúsculo).
    assert_eq!(criada["cnpj"], "11222333000181");
    assert_eq!(criada["email"], "contato@rapidos.com.br");
    assert_eq!(criada["ativo"], true);

    // O recurso criado precisa estar legível em GET /{id}.
    let id = criada["id"].as_str().unwrap();
    let (status_get, corpo_get) = enviar(
        &app,
        Request::builder()
            .method(Method::GET)
            .uri(format!("/transportadoras/{id}"))
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;

    assert_eq!(status_get, StatusCode::OK);
    assert_eq!(corpo_json(&corpo_get)["cnpj"], "11222333000181");
}

// ---------------------------------------------------------------------------
// Cenário 2: CNPJ inválido -> 400
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rejeita_cnpj_invalido_retornando_400() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    // Dígito verificador incorreto.
    let mut payload = payload_valido();
    payload["cnpj"] = json!("11.222.333/0001-82");

    let (status, corpo) = enviar(
        &app,
        requisicao_json(Method::POST, "/transportadoras", &payload),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST, "corpo: {corpo:?}");

    let erro = corpo_json(&corpo);
    assert_eq!(erro["status"], 400);
    assert_eq!(erro["campo"], "cnpj");

    // Nenhuma transportadora deve ter sido persistida.
    let (status_lista, corpo_lista) = enviar(
        &app,
        Request::builder()
            .method(Method::GET)
            .uri("/transportadoras")
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;
    assert_eq!(status_lista, StatusCode::OK);
    assert!(corpo_json(&corpo_lista).as_array().expect("lista").is_empty());

    // Sequência de dígitos repetidos também é inválida por lei.
    let mut payload = payload_valido();
    payload["cnpj"] = json!("00000000000000");
    let (status, _) = enviar(
        &app,
        requisicao_json(Method::POST, "/transportadoras", &payload),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ---------------------------------------------------------------------------
// Cenário 3: e-mail inválido -> 400
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rejeita_email_invalido_retornando_400() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    // Sem "@".
    let mut payload = payload_valido();
    payload["email"] = json!("contato.rapidos.com.br");

    let (status, corpo) = enviar(
        &app,
        requisicao_json(Method::POST, "/transportadoras", &payload),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST, "corpo: {corpo:?}");

    let erro = corpo_json(&corpo);
    assert_eq!(erro["status"], 400);
    // O erro aponta o campo exato, permitindo corrigir só o que está errado.
    assert_eq!(erro["campo"], "email");

    // Domínio sem ponto.
    let mut payload = payload_valido();
    payload["email"] = json!("contato@localhost");
    let (status, _) = enviar(
        &app,
        requisicao_json(Method::POST, "/transportadoras", &payload),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ---------------------------------------------------------------------------
// Cenário 4: duplicidade de CNPJ -> 409
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rejeita_cnpj_duplicado_retornando_409() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    let (status, _) = enviar(
        &app,
        requisicao_json(
            Method::POST,
            "/transportadoras",
            &payload_valido(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // Mesmo CNPJ, com máscara diferente: a normalização para dígitos torna os
    // dois payloads equivalentes para o índice único.
    let mut duplicado = payload_valido();
    duplicado["nome"] = json!("Outra Empresa Ltda");
    duplicado["cnpj"] = json!("11222333000181");

    let (status, corpo) = enviar(
        &app,
        requisicao_json(Method::POST, "/transportadoras", &duplicado),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT, "corpo: {corpo:?}");

    let erro = corpo_json(&corpo);
    assert_eq!(erro["status"], 409);
    // Conflito não é validação de campo: `campo` deve ficar ausente.
    assert!(erro["campo"].is_null());

    // O registro original permanece intacto (não foi sobrescrito).
    let (_, corpo_lista) = enviar(
        &app,
        Request::builder()
            .method(Method::GET)
            .uri("/transportadoras")
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;
    let lista = corpo_json(&corpo_lista);
    assert_eq!(lista.as_array().expect("lista").len(), 1);
    assert_eq!(lista[0]["nome"], "Transportes Rapidos Ltda");
}

// ---------------------------------------------------------------------------
// Cenário 5: busca de ID inexistente -> 404
// ---------------------------------------------------------------------------

#[tokio::test]
async fn busca_id_inexistente_retorna_404() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    let id_inexistente = "00000000-0000-4000-8000-000000000000";
    let (status, corpo) = enviar(
        &app,
        Request::builder()
            .method(Method::GET)
            .uri(format!("/transportadoras/{id_inexistente}"))
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND, "corpo: {corpo:?}");

    let erro = corpo_json(&corpo);
    assert_eq!(erro["status"], 404);
    assert!(erro["erro"].as_str().unwrap().contains("não encontrado"));
}

// ---------------------------------------------------------------------------
// Cenário 6: payload malformado -> 400
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rejeita_payload_malformado_retornando_400() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    // JSON sintaticamente quebrado: o extrator `Json` do Axum recusa com 400
    // **antes** de qualquer regra de negócio, já que o corpo não é desserializável.
    let (status, corpo) = enviar(
        &app,
        requisicao_json_cru(
            Method::POST,
            "/transportadoras",
            r#"{"nome": "Transportes Rapidos Ltda", "cnpj": "#,
        ),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST, "corpo: {corpo:?}");
    // O corpo não precisa ser JSON: a rejeição vem do extrator, não do domínio.
    assert!(!corpo.is_empty());
}

// ---------------------------------------------------------------------------
// Cenário 7: exclusão com sucesso -> 204
// ---------------------------------------------------------------------------

#[tokio::test]
async fn exclui_transportadora_com_sucesso_retornando_204() {
    let app = criar_app_em_memoria().await.expect("app de teste");

    let (status, corpo) = enviar(
        &app,
        requisicao_json(
            Method::POST,
            "/transportadoras",
            &payload_valido(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let id = corpo_json(&corpo)["id"].as_str().unwrap().to_string();

    let (status, corpo_delete) = enviar(
        &app,
        Request::builder()
            .method(Method::DELETE)
            .uri(format!("/transportadoras/{id}"))
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);
    // 204 não pode carregar corpo.
    assert!(
        corpo_delete.is_empty(),
        "esperava corpo vazio, recebeu {corpo_delete:?}"
    );

    // O recurso não existe mais.
    let (status_get, _) = enviar(
        &app,
        Request::builder()
            .method(Method::GET)
            .uri(format!("/transportadoras/{id}"))
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;
    assert_eq!(status_get, StatusCode::NOT_FOUND);

    // Apagar de novo devolve 404, e não 204.
    let (status, _) = enviar(
        &app,
        Request::builder()
            .method(Method::DELETE)
            .uri(format!("/transportadoras/{id}"))
            .body(Body::empty())
            .expect("requisição válida"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
