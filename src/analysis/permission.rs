//! メモリのパーミッションによる解析。
//!
//! 単に実行不可アドレスを `NotCode` とするだけ。

use log::warn;

use crate::address::Address;
use crate::input::Input;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(analysis: &mut Analysis, input: &Input) {
    for addr in Address::all() {
        if !input.permissions()[addr].is_executable() {
            // 既に Code とされていたら警告だけ出す。
            if analysis[addr] == AnalysisKind::Code {
                warn!("address {addr:#04X} is Code and unexecutable");
            } else {
                analysis[addr] = AnalysisKind::NotCode;
            }
        }
    }
}
