//! ラベル振り。
//!
//! コードと確定したアドレスについて命令を取得し、必要に応じて命令の参照先にラベルを振る。
//!
//! 逆アセンブル対象バンク内の命令は必ず調べ、参照先にバンクがロードされていればラベルを振る。
//!
//! 外部バンクの命令は、逆アセンブル対象が固定バンクの場合のみ調べる。
//! (バンク切り替えを考慮したルール。参照先が逆アセンブル対象バンク内の場合のみラベルを振る)

use crate::address::Address;
use crate::assembly::{Label, Labels};
use crate::input::Input;
use crate::op::{Op, Operand};

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(analysis: &Analysis, labels: &mut Labels, input: &Input) {
    let memory = input.memory();
    let target_bank = input.target_bank();
    let target_bank_id = input.target_bank_id();

    // target_bank が固定バンクなら全アドレスを、さもなくば target_bank の範囲内のみを調べる。
    let addrs = if target_bank.is_fixed() {
        Address::all()
    } else {
        target_bank.addr_range().into_iter()
    };

    for addr in addrs {
        if analysis[addr] != AnalysisKind::Code {
            continue;
        }

        if let Ok((op, bank_id)) = memory.fetch_op(addr) {
            let from_target = bank_id == target_bank_id;
            set_needed_labels(labels, input, addr, op, from_target);
        }
    }
}

fn set_needed_labels(labels: &mut Labels, input: &Input, addr: Address, op: Op, from_target: bool) {
    let memory = input.memory();

    // jsr, jmp ind の飛び先は(追跡できるなら)エントリポイントラベルとする。
    // それ以外については通常ラベルとする。
    match op {
        Op::Jsr(dst) => set_label(labels, input, from_target, dst, true),
        Op::JmpInd(ptr) => {
            if let Some((dst, _)) = memory.fetch_addr(ptr) {
                set_label(labels, input, from_target, dst, true);
            }
        }
        _ => match op.operand() {
            Operand::Zp(zp) | Operand::ZpX(zp) | Operand::ZpY(zp) => {
                set_label(labels, input, from_target, Address::from(zp), false);
            }
            Operand::Abs(abs) | Operand::AbsX(abs) | Operand::AbsY(abs) => {
                set_label(labels, input, from_target, abs, false);
            }
            Operand::IndX(zp) => set_label(labels, input, from_target, Address::from(zp), false),
            Operand::IndY(zp) => {
                let ptr = Address::from(zp);
                set_label(labels, input, from_target, ptr, false);
                if let Some((dst, _)) = memory.fetch_addr(ptr) {
                    set_label(labels, input, from_target, dst, false);
                }
            }
            Operand::Rel(rel) => {
                let dst = addr.wrapping_add_unsigned(2_usize).wrapping_add_signed(rel);
                set_label(labels, input, from_target, dst, false);
            }
            _ => {}
        },
    }
}

/// 参照元が逆アセンブル対象バンクの場合、参照先にバンクがロードされていればラベルを振る。
/// 参照元が外部バンクの場合、参照先が逆アセンブル対象バンクならラベルを振る。
fn set_label(
    labels: &mut Labels,
    input: &Input,
    from_target: bool,
    dst: Address,
    entrypoint: bool,
) {
    let dst_bank_id = input.memory().find_bank_id(dst);

    let cond = if from_target {
        dst_bank_id.is_some()
    } else {
        dst_bank_id.map_or(false, |dst_bank_id| dst_bank_id == input.target_bank_id())
    };

    if cond {
        labels.set(dst, Label::new(entrypoint));
    }
}
