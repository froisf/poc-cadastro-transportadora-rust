# 🎯 Roadmap do Portfólio - Cadastro de Transportadoras

**Objetivo:** Construir um portfólio diferenciado mostrando domínio do
legado (RPGLE/JDE) + moderno (Rust/REST), com arquitetura SOLID.

**Meta final:** Impressionar recrutadores técnicos em vagas do
ecossistema IBM i / JDE.

---

## 📊 Status Atual

- [x] POC Rust criada e funcionando
- [x] Refactor SOLID aplicado (via OpenCode)
- [ ] Validação dos endpoints (Fase 1)
- [ ] POC RPGLE revisada e documentada
- [ ] Ponte Rust ↔ RPGLE

---

## 🗺️ FASE 1: Validar o Refactor Rust (30 min)

### Testes
- [ ] Teste 1: GET /health → "ok"
- [ ] Teste 2: GET /transportadoras → lista JSON
- [ ] Teste 3: GET /transportadoras/:id → 1 item
- [ ] Teste 4: GET /transportadoras/nao-existe → 404
- [ ] Teste 5: POST /transportadoras → 201 Created
- [ ] Teste 6: PUT /transportadoras/:id → 200
- [ ] Teste 7: DELETE /transportadoras/:id → 204

---

## 🗺️ FASE 2: Swagger/OpenAPI (1h)

- [ ] Adicionar utoipa e utoipa-swagger-ui no Cargo.toml
- [ ] Anotar structs com #[derive(ToSchema)]
- [ ] Anotar handlers com #[utoipa::path(...)]
- [ ] Criar struct ApiDoc
- [ ] Servir /swagger-ui no main.rs
- [ ] Testar em http://127.0.0.1:3000/swagger-ui

---

## 🗺️ FASE 3: README do Rust (30 min)

- [ ] Título e descrição
- [ ] Stack (Rust, Axum, SQLx, SQLite)
- [ ] Diagrama de arquitetura (SOLID)
- [ ] Como rodar (3 comandos)
- [ ] Tabela de endpoints
- [ ] Exemplos de curl
- [ ] Decisões arquiteturais

---

## 🗺️ FASE 4: Documentar POC RPGLE (1h)

- [ ] Revisar QDDSSRC/QRPGLESRC
- [ ] Testar TRN001.rpgle no IBM i
- [ ] Criar README do RPGLE
- [ ] Comentar código

---

## 🗺️ FASE 5: Ponte Rust ↔ RPGLE (2h) 🌟

- [ ] Definir formato de arquivo (largura fixa, JSON ou XML)
- [ ] Criar módulo legacy/ no Rust
- [ ] Endpoint POST /transportadoras/:id/exportar-legado
- [ ] Programa RPGLE TRN002.rpgle consome o arquivo
- [ ] Documentar a integração

---

## 🗺️ FASE 6: GitHub (30 min)

- [ ] Criar repo principal
- [ ] README comparativo (Rust vs RPGLE)
- [ ] Diagramas de arquitetura
- [ ] Links para os 3 repos

---

## 📅 Cronograma

| Fase | Tempo | Quando |
|---|---|---|
| 1 | 30 min | HOJE |
| 2 | 1h | Próxima sessão |
| 3 | 30 min | Próxima sessão |
| 4 | 1h | Fim de semana |
| 5 | 2h | Fim de semana |
| 6 | 30 min | Final |

**Total:** ~5h30

---

## 🎯 Regras de Ouro

1. Uma coisa por vez
2. Commit antes de mudar
3. Testar antes de avançar
4. Documentar enquanto faz
5. Qualidade > velocidade

---

*Próxima ação: Fase 1 - Teste 1 (abrir http://127.0.0.1:3000/health)*