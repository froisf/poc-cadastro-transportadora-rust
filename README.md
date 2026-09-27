# 🚚 PoC — Modernização de Legado AS/400 (RPGLE) para Rust

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.7-blue)
![SQLx](https://img.shields.io/badge/SQLx-0.7-green)
![SQLite](https://img.shields.io/badge/SQLite-3-blue?logo=sqlite)

> **Prova de Conceito (PoC)** que demonstra a modernização de um sistema legado de **Cadastro de Transportadoras** em **RPGLE / AS/400** para uma **API REST em Rust** com **Axum**, **SQLx** e **SQLite**.

---

## 🎯 Objetivo

Demonstrar a **engenharia reversa** e **modernização** de um sistema monolítico RPGLE do AS/400 para uma arquitetura cloud-native em Rust, mantendo as regras de negócio originais.

📄 **Análise completa:** [ANALISE_LEGADO.md](./ANALISE_LEGADO.md)

---

## 🏗️ Arquitetura

LEGADO (AS/400)              MODERNO (Rust)
TRN001.rpgle    ────▶   HTTP (Swagger)
     ↓                        ↓
TRNPF (PF)              Handlers (Axum)
DB2/400                       ↓
     ↓                   Service
TRNDSP (DSPF)                 ↓
Tela 5250               Repository
                              ↓
                         SQLite

---

## 🚀 Tecnologias

| Camada | Tecnologia |
|---|---|
| Linguagem | Rust 1.75+ |
| Framework Web | Axum 0.7 |
| Runtime | Tokio |
| Banco | SQLite (SQLx) |
| Serialização | Serde |
| Documentação | utoipa / Swagger |
| Testes | cargo test |

---
## 📦 Estrutura

    poc-cadastro-transportadora-rust/
    ├── src/
    │   ├── dtos/              # DTOs (Request/Response)
    │   ├── handlers/          # Endpoints HTTP
    │   ├── models/            # Structs de domínio
    │   ├── repositories/      # Acesso ao banco (SQLx)
    │   ├── services/          # Regras de negócio
    │   ├── error.rs           # Erros customizados
    │   ├── lib.rs
    │   └── main.rs
    ├── tests/                 # Testes de integração
    ├── legacy/                # Fontes AS/400 originais
    │   ├── TRNPF.dds
    │   ├── TRNDSP.dds
    │   └── TRN001.rpgle
    ├── ANALISE_LEGADO.md
    ├── ROADMAP.md
    └── README.md

---

## ⚙️ Como Rodar

### Pré-requisitos

- [Rust](https://rustup.rs/) (1.75+)

### 1. Clone o repositório

    git clone https://github.com/froisf/poc-cadastro-transportadora-rust.git
    cd poc-cadastro-transportadora-rust

### 2. Rode os testes

    cargo test

### 3. Suba o servidor

    cargo run

Acesse o **Swagger UI**: [http://127.0.0.1:3000/swagger-ui/](http://127.0.0.1:3000/swagger-ui/)

---

## 🔌 Endpoints

| Método | Rota | Descrição | Status |
|---|---|---|---|
| GET | /health | Health check | 200 |
| GET | /transportadoras | Lista todas | 200 |
| POST | /transportadoras | Cria nova | 201 / 400 / 409 |
| GET | /transportadoras/{id} | Busca por ID | 200 / 404 |
| PUT | /transportadoras/{id} | Atualiza | 200 / 400 / 404 |
| DELETE | /transportadoras/{id} | Remove | 204 / 404 |

---
## 📥 Exemplo de Requisição

### Criar transportadora (POST)

    curl -X POST http://127.0.0.1:3000/transportadoras \
      -H "Content-Type: application/json" \
      -d '{
        "nome": "Transportes Rápidos Ltda",
        "cnpj": "11.222.333/0001-81",
        "email": "contato@rapidos.com.br",
        "telefone": "1133334444",
        "endereco": "Rua das Flores, 100 - São Paulo/SP"
      }'

**Resposta (201 Created):**

    {
      "id": "16164524-a7a3-4d1f-ac78-f4b2b3ffa0e6",
      "nome": "Transportes Rápidos Ltda",
      "cnpj": "11222333000181",
      "email": "contato@rapidos.com.br",
      "telefone": "1133334444",
      "endereco": "Rua das Flores, 100 - São Paulo/SP",
      "ativo": true
    }

---

## ✅ Regras de Negócio (extraídas do RPGLE)

| Campo | Regra | Origem no Legado |
|---|---|---|
| nome | Entre 2 e 120 caracteres, obrigatório | TRNNOM |
| cnpj | 14 dígitos, DV válidos, único | TRNCNPJ |
| email | Formato válido, lowercase | TRNEML |
| telefone | 10 ou 11 dígitos | — |
| endereco | Obrigatório | — |
| ativo | Boolean (true/false) | TRNSTS (A/I) |

📄 Detalhes completos em [ANALISE_LEGADO.md](./ANALISE_LEGADO.md)

---

## 🧪 Testes

O projeto possui **11 testes** cobrindo os principais cenários:

- Criação com sucesso (201)
- CNPJ inválido (400)
- CNPJ duplicado (409)
- E-mail inválido (400)
- Payload malformado (400)
- Recurso não encontrado (404)
- Exclusão com sucesso (204)
- Isolamento de banco (SQLite em memória)

Rodar os testes:

    cargo test

---

## 🎓 Aprendizados

- Engenharia reversa de **RPGLE free-format** → Rust moderno
- **De-para** entre DDS (TRNPF) e struct Transportadora
- Tradução de validações de ValidaProc → Rust idiomático
- **Arquitetura limpa:** handlers → services → repositories
- Boas práticas REST com **Swagger/OpenAPI**

---

## 👤 Autor

**Flavio Frois** — [@froisf](https://github.com/froisf)

---

⭐ **Se este projeto foi útil para você, deixe uma estrela!**