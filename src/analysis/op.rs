//! 命令単位での解析。
//!
//! 無効とみなした命令を `NotCode` とする。
//! また、`Code` である命令のオペランドは `Code` でない限り `NotCode` とする。
//! (オペランドを実行することはかなり稀なので)

use crate::address::{Address, ZpAddress};
use crate::config::AnalysisConfig;
use crate::input::Input;
use crate::memory::{FetchOpError, Memory};
use crate::op::{Op, OpSucc, Operand};
use crate::permission::Permissions;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(analysis: &mut Analysis, input: &Input, config: &AnalysisConfig) {
    analyze_invalid_op(analysis, input, config);
    analyze_operand(analysis, input.memory());
}

/// `Unknown` な全アドレスについて、その内容が無効な命令なら `NotCode` とする。
fn analyze_invalid_op(analysis: &mut Analysis, input: &Input, config: &AnalysisConfig) {
    for addr in Address::all() {
        if analysis[addr] != AnalysisKind::Unknown {
            continue;
        }

        // 命令を読み取り、それが無効なら NotCode とする。
        // オペコード自体が読み取れない場合は解析不能なので Unknown のままにする。
        // 命令が尻切れになるなら NotCode とする。
        match input.memory().fetch_op(addr) {
            Ok((op, bank_id)) => {
                if is_invalid_op(input, config, addr, op, bank_id) {
                    analysis[addr] = AnalysisKind::NotCode;
                }
            }
            Err(FetchOpError::Nothing) => {}
            Err(FetchOpError::Incomplete(_)) => analysis[addr] = AnalysisKind::NotCode,
        };
    }
}

/// 1 つの `Unknown` なアドレスについて、その内容が無効な命令かどうかを返す。
fn is_invalid_op(
    input: &Input,
    config: &AnalysisConfig,
    addr: Address,
    op: Op,
    bank_id: usize,
) -> bool {
    // NOTE: 後続アドレスが全て実行不可である場合も無効とできるが、
    // この処理は制御フロー解析時にまとめて行う。

    let memory = input.memory();
    let perms = input.permissions();

    op_is_forbidden(config, op)
        || operand_is_wrapping_ptr(op)
        || !op_has_valid_succ(memory, addr, op, bank_id)
        || op_reads_unreadable_addr(perms, op)
        || op_writes_unwritable_addr(perms, op)
}

/// 禁止命令かどうかを返す。
fn op_is_forbidden(config: &AnalysisConfig, op: Op) -> bool {
    // 非公式命令は禁止。
    if !op.is_official() {
        return true;
    }

    // 設定で許可されていない命令は禁止。
    if matches!(op, Op::Brk) && !config.allow_brk() {
        return true;
    }
    if matches!(op, Op::Clv) && !config.allow_clv() {
        return true;
    }
    if matches!(op, Op::Sed) && !config.allow_sed() {
        return true;
    }

    false
}

/// オペランドがページ境界をまたぐポインタかどうかを返す。
fn operand_is_wrapping_ptr(op: Op) -> bool {
    match op.operand() {
        Operand::Ind(abs) => (abs.get() & 0xFF) == 0xFF,
        Operand::IndX(zp) | Operand::IndY(zp) => zp.get() == 0xFF,
        _ => false,
    }
}

/// 命令が有効な後続アドレス(実行後のプログラムカウンタとしてありうる値)を持つかどうかを返す。
fn op_has_valid_succ(memory: &Memory, addr: Address, op: Op, bank_id: usize) -> bool {
    // 後続アドレスが元のアドレスと同一バンクに属するかどうかを返す。
    let is_same_bank = |addr_succ: Address| {
        memory
            .find_bank_id(addr_succ)
            .map_or(false, |bank_id_succ| bank_id_succ == bank_id)
    };

    // アドレス空間内で wrap したり、元のバンク外に出るのは基本的に許さない。
    // 例外として brk, jsr, rti, rts, jmp abs, jmp ind のみ元のバンク外に出ることを許す。
    match op.succ() {
        OpSucc::Normal(offset) => addr
            .checked_add_unsigned(offset)
            .map_or(false, is_same_bank),
        OpSucc::Brk => true,
        OpSucc::Kil => true,
        // 分岐時の行き先がアドレス空間内で wrap したり、元のバンク外に出るのは許さない。
        // 無条件分岐の可能性があるので、非分岐時の行き先は問わない。
        OpSucc::Branch(rel) => addr.checked_add_signed(rel).map_or(false, is_same_bank),
        OpSucc::Jsr(_) => true,
        OpSucc::Rti => true,
        OpSucc::Rts => true,
        OpSucc::JmpAbs(_) => true,
        OpSucc::JmpInd(_) => true,
    }
}

/// 命令がメモリを読み取り、かつそのアドレス候補が全て読み取り不可かどうかを返す。
fn op_reads_unreadable_addr(perms: &Permissions, op: Op) -> bool {
    op_read_candidates(op).map_or(false, |mut it| it.all(|addr| !perms[addr].is_readable()))
}

/// 命令がメモリに書き込み、かつそのアドレス候補が全て書き込み不可かどうかを返す。
fn op_writes_unwritable_addr(perms: &Permissions, op: Op) -> bool {
    op_write_candidates(op).map_or(false, |mut it| it.all(|addr| !perms[addr].is_writable()))
}

/// 命令がメモリを読み取る場合、そのアドレス候補を全て列挙する。
/// 読み取りを行わないなら `None` を返す。
///
/// 「読み取り」はポインタ自体の読み取りも含むが、オペコード/オペランドのフェッチは含まない。
///
/// NOTE: とりあえずページまたぎ時の dummy read は考慮しない。
///
/// NOTE: そこまで高速化する必要はないので、単純にトレイトオブジェクトを返しておく。
fn op_read_candidates(op: Op) -> Option<Box<dyn Iterator<Item = Address>>> {
    let zp_all = ZpAddress::all().map(Address::from);

    // まず間接アドレッシングの場合を考える。
    // たとえ読み取り命令でなくても、ポインタ自体の読み取りは行われることに注意。
    // indirect x, indirect y についてはとりあえずポインタの指す先までは考えない。
    // (実際上、ゼロページの内容は読めないのが普通なので、ここで頑張っても得るものがほぼない)
    // 事前の解析により、ポインタはページ境界をまたがないことが保証されていることに注意。
    //
    // 間接アドレッシング以外の場合、読み取り命令でなければ一切の読み取りが行われない。
    match op.operand() {
        // jmp ind はポインタ自体のみを読み取る。
        Operand::Ind(ptr) => Some(Box::new(
            [ptr, ptr.checked_add_unsigned(1_usize).unwrap()].into_iter(),
        )),
        // indirect x はポインタとしてゼロページの任意のアドレスを読み取りうる。
        Operand::IndX(_) => Some(Box::new(zp_all)),
        // indirect y は必ずポインタ自体を読み取る。
        Operand::IndY(ptr) => Some(Box::new(
            [ptr, ptr.checked_add_unsigned(1_usize).unwrap()]
                .into_iter()
                .map(Address::from),
        )),
        // implied, accumulator, immediate, relative は読み取りを行わない。
        Operand::Imp | Operand::Acc | Operand::Imm(_) | Operand::Rel(_) => None,
        // (読み取り命令) zp は対象アドレスのみを読み取る。
        Operand::Zp(zp) if op.is_read() => Some(Box::new(std::iter::once(Address::from(zp)))),
        // (読み取り命令) zpx, zpy はゼロページの任意のアドレスを読み取りうる。
        Operand::ZpX(_) | Operand::ZpY(_) if op.is_read() => Some(Box::new(zp_all)),
        // (読み取り命令) abs は対象アドレスのみを読み取る。
        Operand::Abs(abs) if op.is_read() => Some(Box::new(std::iter::once(abs))),
        // (読み取り命令) abx, aby はベースアドレス以降の 0x100 バイトのいずれかを読み取りうる。
        Operand::AbsX(abs) | Operand::AbsY(abs) if op.is_read() => Some(Box::new(
            (0..=0xFF).map(move |i| Address::new(abs.get().wrapping_add(i))),
        )),
        // それ以外の場合、読み取り命令でなければ一切の読み取りを行わない。
        _ => None,
    }
}

/// 命令がメモリへ書き込む場合、そのアドレス候補を全て列挙する。
/// 書き込みを行わないなら `None` を返す。
///
/// NOTE: そこまで高速化する必要はないので、単純にトレイトオブジェクトを返しておく。
fn op_write_candidates(op: Op) -> Option<Box<dyn Iterator<Item = Address>>> {
    // 書き込み命令でなければ一切の書き込みが行われない。
    if !op.is_write() {
        return None;
    }

    // indirect x, indirect y についてはとりあえずポインタの指す先までは考えない。
    // (実際上、ゼロページの内容は読めないのが普通なので、ここで頑張っても得るものがほぼない)
    // 事前の解析により、ポインタはページ境界をまたがないことが保証されていることに注意。
    match op.operand() {
        // ポインタの指す先までは考えないので、間接アドレッシングについては書き込みを行わない扱いにする。
        Operand::Ind(_) | Operand::IndX(_) | Operand::IndY(_) => None,
        // implied, accumulator, immediate, relative は書き込みを行わない。
        Operand::Imp | Operand::Acc | Operand::Imm(_) | Operand::Rel(_) => None,
        // zp は対象アドレスのみに書き込む。
        Operand::Zp(zp) => Some(Box::new(std::iter::once(Address::from(zp)))),
        // zpx, zpy はゼロページの任意のアドレスに書き込みうる。
        Operand::ZpX(_) | Operand::ZpY(_) => Some(Box::new(ZpAddress::all().map(Address::from))),
        // abs は対象アドレスのみに書き込む。
        Operand::Abs(abs) => Some(Box::new(std::iter::once(abs))),
        // abx, aby はベースアドレス以降の 0x100 バイトのいずれかに書き込みうる。
        Operand::AbsX(abs) | Operand::AbsY(abs) => Some(Box::new(
            (0..=0xFF).map(move |i| Address::new(abs.get().wrapping_add(i))),
        )),
    }
}

/// `Code` な全アドレスについて、そのオペランドが `Code` でないなら `NotCode` とする。
fn analyze_operand(analysis: &mut Analysis, memory: &Memory) {
    macro_rules! set_notcode {
        ($addr:expr) => {{
            if analysis[$addr] != AnalysisKind::Code {
                analysis[$addr] = AnalysisKind::NotCode;
            }
        }};
    }

    for addr in Address::all() {
        if analysis[addr] != AnalysisKind::Code {
            continue;
        }

        match memory.fetch_op(addr) {
            Ok((op, _)) => {
                for i in 1..op.len().get() {
                    let addr_opr = addr.checked_add_unsigned(i).unwrap();
                    set_notcode!(addr_opr);
                }
            }
            Err(FetchOpError::Nothing) => {}
            Err(FetchOpError::Incomplete(buf)) => {
                for i in 1..buf.len() {
                    let addr_opr = addr.checked_add_unsigned(i).unwrap();
                    set_notcode!(addr_opr);
                }
            }
        }
    }
}
