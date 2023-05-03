use std::fmt::{Display, Formatter};
use std::io::Write;

use crate::address::{Address, ZpAddress};
use crate::assembly::{Assembly, Label, Labels, Statement};
use crate::op::{Op, Operand};

/// ca65 用のアセンブリを出力する。
pub fn output_assembly<W: Write>(wtr: &mut W, asm: &Assembly) -> anyhow::Result<()> {
    out_preamble(wtr, asm)?;
    out_statements(wtr, asm)?;

    Ok(())
}

/// アセンブリ先頭の `.org` 宣言やラベル定義などを出力する。
fn out_preamble<W: Write>(wtr: &mut W, asm: &Assembly) -> anyhow::Result<()> {
    // .segment 宣言。
    writeln!(
        wtr,
        ";---------------------------------------------------------------------"
    )?;
    writeln!(wtr, r#".segment "{}""#, asm.bank_name())?;
    writeln!(
        wtr,
        ";---------------------------------------------------------------------"
    )?;
    writeln!(wtr)?;

    // アセンブリのアドレス範囲外のラベルを定義。
    let mut defined_label = false;
    for addr in Address::all() {
        if asm.labels().get(addr).is_some() && !asm.bank_addr_range().contains_addr(addr) {
            defined_label = true;
            writeln!(wtr, "{} := {}", LabelAddr(addr), HexAddr(addr))?;
        }
    }
    if defined_label {
        writeln!(wtr)?;
    }

    Ok(())
}

/// アセンブリの文たちを出力する。
fn out_statements<W: Write>(wtr: &mut W, asm: &Assembly) -> anyhow::Result<()> {
    let mut addr = asm.bank_addr();
    let mut stmt_pre: Option<Statement> = None;

    for stmt in asm.statements() {
        // 必要に応じて空行を挿入。
        if stmt_pre.as_ref().map_or(false, |stmt_pre| {
            needs_blank_line(asm.labels(), addr, stmt_pre, stmt)
        }) {
            writeln!(wtr)?;
        }

        out_statement(wtr, asm, addr, stmt)?;

        let Some(addr_nxt) = addr.checked_add_unsigned(stmt.len()) else {
            break;
        };

        addr = addr_nxt;
        stmt_pre = Some(stmt.clone());
    }

    Ok(())
}

fn out_statement<W: Write>(
    wtr: &mut W,
    asm: &Assembly,
    addr: Address,
    stmt: &Statement,
) -> anyhow::Result<()> {
    // addr にエントリポイントラベルがあればコメント欄を挿入。
    let entrypoint = asm.labels().get(addr).map_or(false, Label::is_entrypoint);
    if entrypoint {
        writeln!(wtr, ";;; ")?;
    }

    // stmt の範囲内いずれかにラベルがあれば addr にラベルを振る必要がある。
    let need_label = (0..stmt.len().get()).any(|i| {
        asm.labels()
            .get(addr.checked_add_unsigned(i).unwrap())
            .is_some()
    });
    if need_label {
        writeln!(wtr, "{}:", LabelAddr(addr))?;
    }
    // stmt の途中 (addr 以外) にラベルがあるなら、addr のラベルからの相対位置として定義する。
    for i in 1..stmt.len().get() {
        let addr_mid = addr.checked_add_unsigned(i).unwrap();
        if asm.labels().get(addr_mid).is_some() {
            writeln!(wtr, "{} := {} + {i}", LabelAddr(addr_mid), LabelAddr(addr))?;
        }
    }

    match *stmt {
        Statement::Op(op) => out_op(wtr, asm, addr, op)?,
        Statement::IncompleteOp(ref buf) => out_incomplete_op(wtr, buf)?,
        Statement::Byte(b) => out_byte(wtr, b)?,
    }

    Ok(())
}

fn out_op<W: Write>(wtr: &mut W, asm: &Assembly, addr: Address, op: Op) -> anyhow::Result<()> {
    if op.is_official() {
        writeln!(wtr, "        {}", FormatOp::new(asm.labels(), addr, op))?;
    } else {
        // 非公式命令の場合、ca65 でサポートされていないものもあるため、
        // コメントを付けた上で単なるバイト列として出力する。
        writeln!(wtr, "        ; {}", FormatOp::new(asm.labels(), addr, op))?;
        for b in op.to_bytes() {
            out_byte(wtr, b)?;
        }
    }

    Ok(())
}

fn out_incomplete_op<W: Write>(wtr: &mut W, buf: &[u8]) -> anyhow::Result<()> {
    writeln!(wtr, "        ; INCOMPLETE OP")?;

    for &b in buf {
        out_byte(wtr, b)?;
    }

    Ok(())
}

fn out_byte<W: Write>(wtr: &mut W, b: u8) -> anyhow::Result<()> {
    writeln!(wtr, "        .byte   {}", HexU8(b))?;

    Ok(())
}

/// 2 つの文の間に空行を入れるべきかどうかを返す。
fn needs_blank_line(
    labels: &Labels,
    addr: Address,
    stmt_pre: &Statement,
    stmt: &Statement,
) -> bool {
    // コード/データ境界なら空行を入れる。
    if matches!(
        (stmt_pre, stmt),
        (
            Statement::Op(_) | Statement::IncompleteOp(_),
            Statement::Byte(_)
        ) | (
            Statement::Byte(_),
            Statement::Op(_) | Statement::IncompleteOp(_)
        )
    ) {
        return true;
    }

    // 現在の文がエントリポイントなら空行を入れる。
    if labels
        .get(addr)
        .map_or(false, |label| label.is_entrypoint())
    {
        return true;
    }

    // 直前の文がコード終端 (rti, rts, jmp) なら空行を入れる。
    if matches!(
        stmt_pre,
        Statement::Op(Op::Rti | Op::Rts | Op::JmpAbs(_) | Op::JmpInd(_))
    ) {
        return true;
    }

    false
}

/// 命令を ca65 形式にフォーマットする。
/// NOTE: 非公式命令の場合、必ずしも ca65 上で正しい表現になるとは限らない。
#[derive(Debug)]
struct FormatOp<'a> {
    labels: &'a Labels,
    addr: Address,
    op: Op,
}

impl<'a> FormatOp<'a> {
    fn new(labels: &'a Labels, addr: Address, op: Op) -> Self {
        Self { labels, addr, op }
    }
}

impl Display for FormatOp<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mne = self.op.opcode().mnemonic();

        match self.op.operand() {
            Operand::Imp => write!(f, "{mne}"),
            Operand::Acc => write!(f, "{mne}"),
            Operand::Imm(imm) => write!(f, "{mne}     #{}", ResolveImm::new(self.op, imm)),
            Operand::Zp(zp) => write!(f, "{mne}     {}", ResolveZpAddr::new(self.labels, zp)),
            Operand::ZpX(zp) => write!(f, "{mne}     {},x", ResolveZpAddr::new(self.labels, zp)),
            Operand::ZpY(zp) => write!(f, "{mne}     {},y", ResolveZpAddr::new(self.labels, zp)),
            Operand::Abs(abs) => write!(f, "{mne}     {}", ResolveAbsAddr::new(self.labels, abs)),
            Operand::AbsX(abs) => {
                write!(f, "{mne}     {},x", ResolveAbsAddr::new(self.labels, abs))
            }
            Operand::AbsY(abs) => {
                write!(f, "{mne}     {},y", ResolveAbsAddr::new(self.labels, abs))
            }
            Operand::Ind(abs) => write!(f, "{mne}     ({})", ResolveAddr::new(self.labels, abs)),
            Operand::IndX(zp) => write!(f, "{mne}     ({},x)", ResolveZpAddr::new(self.labels, zp)),
            Operand::IndY(zp) => write!(f, "{mne}     ({}),y", ResolveZpAddr::new(self.labels, zp)),
            Operand::Rel(rel) => {
                let dst = self
                    .addr
                    .wrapping_add_unsigned(2_usize)
                    .wrapping_add_signed(rel);
                write!(f, "{mne}     {}", ResolveAddr::new(self.labels, dst))
            }
        }
    }
}

/// `Address` を文字列化する。
/// 対応するラベルがあればラベル文字列にし、さもなくば 16 進フォーマットする。
/// 16 進フォーマット時の桁数はゼロページなら 2, さもなくば 4 とする。
#[derive(Debug)]
struct ResolveAddr<'a> {
    labels: &'a Labels,
    abs: Address,
}

impl<'a> ResolveAddr<'a> {
    fn new(labels: &'a Labels, abs: Address) -> Self {
        Self { labels, abs }
    }
}

impl Display for ResolveAddr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.labels.get(self.abs).is_some() {
            LabelAddr(self.abs).fmt(f)
        } else {
            HexAddr(self.abs).fmt(f)
        }
    }
}

/// `Address` を文字列化する。
/// 基本的に `ResolveAddress` と同じだが、必要に応じて "a:" プレフィックスを付ける。
#[derive(Debug)]
struct ResolveAbsAddr<'a> {
    labels: &'a Labels,
    abs: Address,
}

impl<'a> ResolveAbsAddr<'a> {
    fn new(labels: &'a Labels, abs: Address) -> Self {
        Self { labels, abs }
    }
}

impl Display for ResolveAbsAddr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.abs.is_zeropage() {
            f.write_str("a:")?;
        }

        if self.labels.get(self.abs).is_some() {
            LabelAddr(self.abs).fmt(f)
        } else {
            HexAddr(self.abs).fmt(f)
        }
    }
}

/// `ZpAddress` を文字列化する。
/// 対応するラベルがあればラベル文字列にし、さもなくば 16 進フォーマットする (2 桁)。
#[derive(Debug)]
struct ResolveZpAddr<'a> {
    labels: &'a Labels,
    zp: ZpAddress,
}

impl<'a> ResolveZpAddr<'a> {
    fn new(labels: &'a Labels, zp: ZpAddress) -> Self {
        Self { labels, zp }
    }
}

impl Display for ResolveZpAddr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.labels.get(Address::from(self.zp)).is_some() {
            LabelAddr(Address::from(self.zp)).fmt(f)
        } else {
            HexZpAddr(self.zp).fmt(f)
        }
    }
}

/// `u8` 型の即値を文字列化する。
///
/// 小さい即値は 10 進フォーマットする(ビット演算命令かどうかで閾値を変える)。
#[derive(Debug)]
struct ResolveImm {
    op: Op,
    imm: u8,
}

impl ResolveImm {
    fn new(op: Op, imm: u8) -> Self {
        Self { op, imm }
    }
}

impl Display for ResolveImm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let decimal = if self.op.is_bitop_imm() {
            self.imm <= 9
        } else {
            self.imm <= 16
        };

        if decimal {
            self.imm.fmt(f)
        } else {
            HexU8(self.imm).fmt(f)
        }
    }
}

/// 指定したアドレス用のラベル文字列を作る。
#[derive(Debug)]
struct LabelAddr(Address);

impl Display for LabelAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "L_{:04X}", self.0)
    }
}

/// `Address` を ca65 用に 16 進フォーマットする。
/// 桁数はゼロページなら 2 桁、さもなくば 4 桁。
#[derive(Debug)]
struct HexAddr(Address);

impl Display for HexAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_zeropage() {
            write!(f, "${:02X}", self.0)
        } else {
            write!(f, "${:04X}", self.0)
        }
    }
}

/// `ZpAddress` を ca65 用に 16 進フォーマットする (2 桁)。
#[derive(Debug)]
struct HexZpAddr(ZpAddress);

impl Display for HexZpAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}", self.0)
    }
}

/// `u8` を ca65 用に 16 進フォーマットする。
#[derive(Debug)]
struct HexU8(u8);

impl Display for HexU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}", self.0)
    }
}
