//! CDL による解析。
//!
//! これは一番最初に実行され、結果は他のいかなる解析より優先される。

use crate::address::Address;
use crate::assembly::{Label, Labels};
use crate::input::Input;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(analysis: &mut Analysis, labels: &mut Labels, input: &Input) {
    let cdl = input.cdl();

    // 全アドレスに対し CDL による解析を行う。
    for addr in Address::all() {
        // CDL でオペコードとされているアドレスは Code とする。
        if cdl.is_opcode(addr) {
            analysis[addr] = AnalysisKind::Code;
        }

        // CDL でオペコードでなく、かつデータとされているアドレスは NotCode とする。
        if !cdl.is_opcode(addr) && cdl.is_data(addr) {
            analysis[addr] = AnalysisKind::NotCode;
        }

        // 逆アセンブル対象バンク内に限り、必要に応じてラベルを振る。
        if input.target_bank().contains_addr(addr) {
            let need_label = cdl.is_jump_target(addr)
                || cdl.is_entrypoint(addr)
                || cdl.is_indirect_data_start(addr)
                || cdl.is_pcm_data_start(addr);
            if need_label {
                let entrypoint = cdl.is_entrypoint(addr);
                labels.set(addr, Label::new(entrypoint));
            }
        }
    }
}
