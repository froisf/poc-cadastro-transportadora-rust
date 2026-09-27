use crate::{
    dtos::{AtualizarTransportadora, CriarTransportadora},
    error::{AppError, AppResult},
    models::Transportadora,
    repositories::TransportadoraRepository,
};

/// Caso de uso de `Transportadora`.
///
/// O repositório entra por **generics** (injeção de dependência em tempo de
/// compilação): o Service conhece apenas a interface `TransportadoraRepository`.
#[derive(Debug, Clone)]
pub struct TransportadoraService<R: TransportadoraRepository> {
    repository: R,
}

impl<R: TransportadoraRepository> TransportadoraService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Caso de uso: listar todas as transportadoras.
    pub async fn listar(&self) -> AppResult<Vec<Transportadora>> {
        self.repository.listar().await
    }

    /// Caso de uso: buscar por id (404 quando não existe).
    pub async fn buscar_por_id(&self, id: &str) -> AppResult<Transportadora> {
        self.repository
            .buscar_por_id(id)
            .await?
            .ok_or_else(|| AppError::nao_encontrado("transportadora"))
    }

    /// Caso de uso: criar. Valida os dados antes de persistir.
    pub async fn criar(&self, dto: CriarTransportadora) -> AppResult<Transportadora> {
        validar_campos(
            &dto.nome,
            &dto.cnpj,
            &dto.email,
            &dto.telefone,
            &dto.endereco,
        )?;

        let transportadora = Transportadora::nova(
            dto.nome.trim().to_string(),
            normalizar_cnpj(&dto.cnpj),
            dto.email.trim().to_lowercase(),
            dto.telefone.trim().to_string(),
            dto.endereco.trim().to_string(),
        );

        self.repository.criar(&transportadora).await
    }

    /// Caso de uso: atualizar (substituição total). Retorna 404 quando o id não existe.
    pub async fn atualizar(
        &self,
        id: &str,
        dto: AtualizarTransportadora,
    ) -> AppResult<Transportadora> {
        validar_campos(
            &dto.nome,
            &dto.cnpj,
            &dto.email,
            &dto.telefone,
            &dto.endereco,
        )?;

        // Garante o 404 antes de tentar gravar.
        self.buscar_por_id(id).await?;

        let alterada = Transportadora {
            id: id.to_string(),
            nome: dto.nome.trim().to_string(),
            cnpj: normalizar_cnpj(&dto.cnpj),
            email: dto.email.trim().to_lowercase(),
            telefone: dto.telefone.trim().to_string(),
            endereco: dto.endereco.trim().to_string(),
            ativo: true,
        };

        self.repository
            .atualizar(id, &alterada)
            .await?
            .ok_or_else(|| AppError::nao_encontrado("transportadora"))
    }

    /// Caso de uso: deletar (404 quando o id não existe).
    pub async fn deletar(&self, id: &str) -> AppResult<()> {
        if self.repository.deletar(id).await? {
            Ok(())
        } else {
            Err(AppError::nao_encontrado("transportadora"))
        }
    }
}

/// Validação de negócio compartilhada por `criar` e `atualizar`.
fn validar_campos(
    nome: &str,
    cnpj: &str,
    email: &str,
    telefone: &str,
    endereco: &str,
) -> AppResult<()> {
    let tamanho_nome = nome.trim().chars().count();
    if !(2..=120).contains(&tamanho_nome) {
        return Err(AppError::validacao(
            "nome",
            "deve ter entre 2 e 120 caracteres",
        ));
    }

    if !cnpj_valido(cnpj) {
        return Err(AppError::validacao(
            "cnpj",
            "dígitos verificadores inválidos",
        ));
    }

    let email = email.trim();
    let partes_email: Vec<&str> = email.split('@').collect();
    if partes_email.len() != 2 || partes_email[0].is_empty() || !partes_email[1].contains('.') {
        return Err(AppError::validacao("email", "formato inválido"));
    }

    let digitos_telefone = telefone.chars().filter(|c| c.is_ascii_digit()).count();
    if !(10..=11).contains(&digitos_telefone) {
        return Err(AppError::validacao(
            "telefone",
            "deve conter DDD + número (10 ou 11 dígitos)",
        ));
    }

    if endereco.trim().is_empty() {
        return Err(AppError::validacao("endereco", "não pode ser vazio"));
    }

    Ok(())
}

/// Remove a máscara do CNPJ, persistindo apenas dígitos (forma canônica).
///
/// Recebe um CNPJ já aprovado por `cnpj_valido` — os casos de uso sempre chamam
/// `validar_campos` antes de normalizar —, portanto os únicos separadores
/// possíveis são os da máscara.
fn normalizar_cnpj(cnpj: &str) -> String {
    cnpj.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Aceita apenas dígitos e os separadores da máscara do CNPJ (`.`, `/`, `-`),
/// tolerando espaços nas pontas. Qualquer outro caractere reprova o CNPJ.
fn mascara_valida(cnpj: &str) -> bool {
    cnpj.trim()
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | '/' | '-'))
}

/// Valida o CNPJ pelos dígitos verificadores (módulo 11).
fn cnpj_valido(cnpj: &str) -> bool {
    // Caracteres fora da máscara são rejeitados, nunca descartados em silêncio:
    // sem esta checagem "11.222.333/0001-8a" passaria como "11.222.333/0001-81".
    if !mascara_valida(cnpj) {
        return false;
    }

    let digitos: Vec<u32> = normalizar_cnpj(cnpj)
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    // CNPJ sempre possui 14 dígitos.
    if digitos.len() != 14 {
        return false;
    }

    // Sequências de dígitos iguais passam no cálculo, mas são inválidas por lei.
    if digitos[..12].iter().all(|&d| d == digitos[0]) {
        return false;
    }

    // O 2º dígito verificador é calculado sobre os 13 dígitos (12 + o DV1).
    if digito_verificador(&digitos[..12], &PESOS_DV1) != digitos[12] {
        return false;
    }

    digito_verificador(&digitos[..13], &PESOS_DV2) == digitos[13]
}

/// Pesos do 1º dígito verificador (12 casas).
const PESOS_DV1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
/// Pesos do 2º dígito verificador (13 casas, já inclui o DV1).
const PESOS_DV2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// Calcula um dígito verificador: soma ponderada, resto da divisão por 11,
/// subtração de 11 e, se o resultado for >= 10, o dígito é 0.
fn digito_verificador(digitos: &[u32], pesos: &[u32]) -> u32 {
    let soma: u32 = digitos
        .iter()
        .zip(pesos.iter())
        .map(|(digito, peso)| digito * peso)
        .sum();

    let resto = soma % 11;
    let digito = 11 - resto;

    if digito >= 10 { 0 } else { digito }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aceita_cnpj_valido() {
        assert!(cnpj_valido("11.222.333/0001-81"));
        assert!(cnpj_valido("11222333000181"));
        // Regressão: CNPJ do relato original deve ser aceito.
        assert!(cnpj_valido("00.000.000/0001-91"));
        assert!(cnpj_valido("00.000.000/0001-91 ")); // espaços nas pontas
    }

    #[test]
    fn rejeita_cnpj_invalido() {
        assert!(!cnpj_valido("11.222.333/0001-82")); // DV errado
        assert!(!cnpj_valido("00000000000000")); // sequência repetida
        assert!(!cnpj_valido("123")); // tamanho inválido
        assert!(!cnpj_valido("abcdefghijklmn")); // sem dígitos
    }

    #[test]
    fn rejeita_caractere_fora_da_mascara() {
        // Antes esses casos eram aceitos: o caractere era descartado em silêncio.
        assert!(!cnpj_valido("11.222.333/0001-8a")); // dígito + letra
        assert!(!cnpj_valido("11.222.333/0001-81x")); // letra no fim
        assert!(!cnpj_valido("11.222.333/0001-8 1")); // espaço no meio
        assert!(!cnpj_valido("112223330001#1"));
        assert!(!cnpj_valido("11,222,333/0001-81")); // máscara com vírgulas
    }

    #[test]
    fn normaliza_mascara() {
        assert_eq!(normalizar_cnpj("11.222.333/0001-81"), "11222333000181");
    }
}