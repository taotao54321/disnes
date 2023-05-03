//! linear sweep 解析。
//!
//! 逆アセンブル対象バンクを先頭からなめて解析し、`Statement` 配列に変換する。
//! (原始的な逆アセンブラと同じ要領)
//! 必要に応じてラベル振りも行う(コード/データ境界など)。

use crate::address::Address;
use crate::assembly::{Label, Labels, Statement};
use crate::input::Input;
use crate::memory::FetchOpError;
use crate::op::Op;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(
    analysis: &mut Analysis,
    labels: &mut Labels,
    input: &Input,
) -> Vec<Statement> {
    let bank = input.target_bank();

    let mut addr = bank.addr();
    let mut stmts = Vec::<Statement>::new();

    loop {
        let stmt = get_stmt(analysis, input, addr);
        let stmt_len = stmt.len();

        // 現在位置がバンク先頭であるか、もしくは特定条件を満たすならラベルを振る。
        // 逆アセンブル対象バンク内であることは確定していることに注意。
        if stmts
            .last()
            .map_or(true, |stmt_pre| needs_label(stmt_pre, &stmt))
        {
            labels.set(addr, Label::new(false));
        }

        stmts.push(stmt);

        // バンク外に出たら終了。
        let Some(addr_nxt) = addr.checked_add_unsigned(stmt_len) else {
            break;
        };
        if !bank.contains_addr(addr_nxt) {
            break;
        }

        addr = addr_nxt;
    }

    stmts
}

/// 指定したアドレスについて `Code`, `NotCode` を確定させた上で対応する `Statement` を返す。
fn get_stmt(analysis: &mut Analysis, input: &Input, addr: Address) -> Statement {
    let memory = input.memory();

    match analysis[addr] {
        AnalysisKind::Unknown => {
            // Unknown は基本的に Code 扱いとするが、命令が尻切れになるなら NotCode とする。
            match memory.fetch_op(addr) {
                Ok((op, _)) => {
                    analysis[addr] = AnalysisKind::Code;
                    Statement::Op(op)
                }
                Err(FetchOpError::Incomplete(buf)) => {
                    analysis[addr] = AnalysisKind::NotCode;
                    Statement::Byte(buf[0])
                }
                Err(FetchOpError::Nothing) => unreachable!(),
            }
        }
        AnalysisKind::Code => {
            // Code の場合、基本的に Op を返すが、バンク末尾で命令が尻切れになる場合、IncompleteOp を返す。
            match memory.fetch_op(addr) {
                Ok((op, _)) => Statement::Op(op),
                Err(FetchOpError::Incomplete(buf)) => Statement::IncompleteOp(buf),
                Err(FetchOpError::Nothing) => unreachable!(),
            }
        }
        AnalysisKind::NotCode => Statement::Byte(memory.get_byte(addr).unwrap().0),
    }
}

/// 直前の文と現在の文が与えられたとき、現在の文にラベルを振るべきかどうかを返す。
fn needs_label(stmt_pre: &Statement, stmt: &Statement) -> bool {
    // コード/データ境界ならラベルを振る。
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

    // 直前の文がコード終端 (rti, rts, jmp) ならラベルを振る。
    if matches!(
        stmt_pre,
        Statement::Op(Op::Rti | Op::Rts | Op::JmpAbs(_) | Op::JmpInd(_))
    ) {
        return true;
    }

    false
}
