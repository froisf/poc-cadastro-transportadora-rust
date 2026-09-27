# Análise de Legado — Cadastro de Transportadoras (AS/400 → Rust)

Documento gerado pela engenharia reversa do programa RPGLE `TRN001` para a PoC em Rust com Axum/SQLx.

---

## 1. Contexto do Legado

| Item | Valor |
|---|---|
| Programa | `TRN001` (RPGLE free-format) |
| Tabela física | `TRNPF` |
| Display File | `TRNDSP` |
| Biblioteca fonte | `QRPGLESRC` / `QDDSRC` |
| Finalidade | Cadastro (CRUD) de Transportadoras com subfile de listagem |

### 1.1 Arquivos utilizados

```rpgle
Dcl-F TRNPF      Usage(*Input:*Output:*Update:*Delete) Keyed UsrOpn;
Dcl-F TRNDSP     WorkStn IndVar(Ind) SFile(TRNSFL:TrnRrn);