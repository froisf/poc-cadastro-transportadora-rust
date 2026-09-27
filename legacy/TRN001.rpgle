**FREE
// ============================================================
// Programa : TRN001
// Descrição: Cadastro de Transportadoras (CRUD completo)
// Autor    : Flavio Frois (versão fictícia para PoC)
// Data     : 2026-09-23
// Observação: Este código é 100% fictício e criado apenas para
//             fins de estudo de engenharia reversa com IA.
// ============================================================

Ctl-Opt DftActGrp(*No) ActGrp(*Caller) Main(TRN001);

// ---------- Arquivos ----------
Dcl-F TRNPF      Usage(*Input:*Output:*Update:*Delete) Keyed UsrOpn;
Dcl-F TRNDSP     WorkStn IndVar(Ind) SFile(TRNSFL:TrnRrn);

// ---------- Estruturas de Dados ----------
Dcl-Ds TrnDs        ExtName('TRNPF') Qualified End-Ds;
Dcl-Ds TrnDsOut     ExtName('TRNPF') Qualified End-Ds;

Dcl-Ds IndDs;
  Exit      Ind Pos(3);
  Cancel    Ind Pos(12);
  SflDsp    Ind Pos(30);
  SflDspCtl Ind Pos(31);
  SflClr    Ind Pos(32);
  SflEnd    Ind Pos(33);
  SflDel    Ind Pos(34);
End-Ds;

// ---------- Variáveis ----------
Dcl-S wModo      Char(1) Inz('L');
Dcl-S wErro      Ind    Inz(*Off);
Dcl-S wMsgErro   Char(70) Inz(*Blanks);
Dcl-S wRrn       Packed(4:0);
Dcl-S wContador  Packed(4:0);
Dcl-S wData      Char(8);
Dcl-S wHora      Char(6);

// ============================================================
// Programa Principal
// ============================================================
Dcl-Proc TRN001;
  Dcl-Pi *N;
  End-Pi;

  Open TRNPF;
  Open TRNDSP;

  wModo = 'L';
  Dow (not Exit);
    Select;
      When wModo = 'L';
        ExSr CarregaLista;
      When wModo = 'I';
        ExSr IncluirRegistro;
      When wModo = 'A';
        ExSr AlterarRegistro;
      When wModo = 'E';
        ExSr ExcluirRegistro;
    EndSl;

    If wErro;
      ExSr MostrarErro;
    EndIf;
  EndDo;

  Close TRNPF;
  Close TRNDSP;
  Return;
End-Proc;

// ============================================================
// Sub-rotina: Carregar lista de transportadoras (subfile)
// ============================================================
Dcl-Proc CarregaLista;
  Dcl-Pi *N;
  End-Pi;

  SflClr    = *On;
  SflDspCtl = *On;
  Write TRNCTL;
  SflClr    = *Off;
  SflDspCtl = *Off;

  SetLL *Start TRNPF;
  Read TRNPF;
  wContador = 0;

  Dow (not %Eof(TRNPF));
    wRrn = wRrn + 1;
    TrnDsOut = TrnDs;
    Write TRNSFL;
    wContador = wContador + 1;
    Read TRNPF;
  EndDo;

  If wContador = 0;
    wMsgErro = 'Nenhuma transportadora cadastrada.';
  EndIf;

  SflDsp    = *On;
  SflDspCtl = *On;
  SflEnd    = *On;
  ExFmt TRNCTL;

  If Exit;
    Return;
  EndIf;

  If wRrn > 0;
    Chain wRrn TRNPF;
    If %Found(TRNPF);
      wModo = 'A';
    EndIf;
  EndIf;
End-Proc;

// ============================================================
// Sub-rotina: Incluir novo registro
// ============================================================
Dcl-Proc IncluirRegistro;
  Dcl-Pi *N;
  End-Pi;

  Clear TrnDsOut;
  TrnDsOut.TRNSTS = 'A';
  wMsgErro = *Blanks;

  ExFmt TRNDTL;

  If Cancel;
    wModo = 'L';
    Return;
  EndIf;

  ExSr ValidaDados;
  If wErro;
    Return;
  EndIf;

  TrnDsOut.TRNCOD = GeraCodigo();

  wData = %Char(%Date():*ISO0);
  wHora = %Char(%Time():*HMS0);
  TrnDsOut.TRNDTC = %Dec(wData:8:0);
  TrnDsOut.TRNDTA = %Dec(wData:8:0);

  Write TRNPF TrnDsOut;
  wMsgErro = 'Transportadora cadastrada com sucesso!';
  wModo = 'L';
End-Proc;

// ============================================================
// Sub-rotina: Alterar registro existente
// ============================================================
Dcl-Proc AlterarRegistro;
  Dcl-Pi *N;
  End-Pi;

  Chain TrnDs.TRNCOD TRNPF;
  If not %Found(TRNPF);
    wMsgErro = 'Registro não encontrado.';
    wModo = 'L';
    Return;
  EndIf;

  TrnDsOut = TrnDs;
  wMsgErro = *Blanks;

  ExFmt TRNDTL;

  If Cancel;
    wModo = 'L';
    Return;
  EndIf;

  ExSr ValidaDados;
  If wErro;
    Return;
  EndIf;

  wData = %Char(%Date():*ISO0);
  TrnDsOut.TRNDTA = %Dec(wData:8:0);

  Update TRNPF TrnDsOut;
  wMsgErro = 'Transportadora alterada com sucesso!';
  wModo = 'L';
End-Proc;

// ============================================================
// Sub-rotina: Excluir registro
// ============================================================
Dcl-Proc ExcluirRegistro;
  Dcl-Pi *N;
  End-Pi;

  Chain TrnDs.TRNCOD TRNPF;
  If not %Found(TRNPF);
    wMsgErro = 'Registro não encontrado.';
    wModo = 'L';
    Return;
  EndIf;

  TrnDsOut = TrnDs;
  wMsgErro = 'Confirme a exclusão com F4 ou cancele com F12.';

  ExFmt TRNDTL;

  If Cancel;
    wModo = 'L';
    Return;
  EndIf;

  Delete TRNPF;
  wMsgErro = 'Transportadora excluída com sucesso!';
  wModo = 'L';
End-Proc;

// ============================================================
// Sub-rotina: Validar dados informados
// ============================================================
Dcl-Proc ValidaDados;
  Dcl-Pi *N;
  End-Pi;

  wErro = *Off;
  wMsgErro = *Blanks;

  If %Trim(TrnDsOut.TRNNOM) = '';
    wMsgErro = 'Razão Social é obrigatória.';
    wErro = *On;
    Return;
  EndIf;

  If %Trim(TrnDsOut.TRNCNPJ) = '';
    wMsgErro = 'CNPJ é obrigatório.';
    wErro = *On;
    Return;
  EndIf;

  If %Len(%Trim(TrnDsOut.TRNCNPJ)) < 14;
    wMsgErro = 'CNPJ deve conter 14 dígitos.';
    wErro = *On;
    Return;
  EndIf;

  If %Len(%Trim(TrnDsOut.TRNUF)) <> 2;
    wMsgErro = 'UF deve conter 2 caracteres.';
    wErro = *On;
    Return;
  EndIf;

  If TrnDsOut.TRNSTS <> 'A' and TrnDsOut.TRNSTS <> 'I';
    wMsgErro = 'Status deve ser A (Ativo) ou I (Inativo).';
    wErro = *On;
    Return;
  EndIf;

  If %Scan('@':TrnDsOut.TRNEML) = 0;
    wMsgErro = 'E-mail inválido (falta @).';
    wErro = *On;
    Return;
  EndIf;
End-Proc;

// ============================================================
// Sub-rotina: Gerar próximo código sequencial
// ============================================================
Dcl-Proc GeraCodigo;
  Dcl-Pi *N Char(10);
  End-Pi;

  Dcl-S wUltimo Char(10);
  Dcl-S wNumero Packed(10:0);

  wUltimo = '0000000000';
  SetGT *Hival TRNPF;
  ReadP TRNPF;
  If not %Eof(TRNPF);
    wUltimo = TrnDs.TRNCOD;
  EndIf;

  Monitor;
    wNumero = %Dec(wUltimo:10:0) + 1;
  On-Error;
    wNumero = 1;
  EndMon;

  Return %Char(wNumero);
End-Proc;

// ============================================================
// Sub-rotina: Mostrar mensagem de erro
// ============================================================
Dcl-Proc MostrarErro;
  Dcl-Pi *N;
  End-Pi;

  ExFmt TRNDTL;
  wErro = *Off;
End-Proc;