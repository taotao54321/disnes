//! 割り込みベクタによる解析。
//!
//! 設定で指定された割り込みについて以下の処理を行う:
//!
//! * 割り込みベクタそのものは(元々 `Code` 指定されていなければ) `NotCode` とする。
//!   必要ならラベルも振る。
//! * 割り込みハンドラのアドレスを取得できれば、それを `Code` とする。
//!   必要ならエントリポイントラベルも振る。

use std::num::NonZeroUsize;

use log::warn;

use crate::address::{Address, AddressRange};
use crate::assembly::{Label, Labels};
use crate::config::AnalysisConfig;
use crate::input::Input;

use super::{Analysis, AnalysisKind};

pub(super) fn analyze(
    analysis: &mut Analysis,
    labels: &mut Labels,
    input: &Input,
    config: &AnalysisConfig,
) {
    // 設定に応じて割り込みハンドラの認識を試みる。

    if config.use_nmi() {
        set_handler(analysis, labels, input, Address::new(0xFFFA), "NMI");
    }
    if config.use_reset() {
        set_handler(analysis, labels, input, Address::new(0xFFFC), "RESET");
    }
    if config.use_irq() {
        set_handler(analysis, labels, input, Address::new(0xFFFE), "IRQ");
    }
}

fn set_handler(
    analysis: &mut Analysis,
    labels: &mut Labels,
    input: &Input,
    ptr: Address,
    name: &str,
) {
    // 割り込みベクタそのものは元々 Code 指定されていなければ NotCode とする。
    {
        let range = AddressRange::from_start_len(ptr, NonZeroUsize::new(2).unwrap());
        if analysis[range].iter().all(|&e| e != AnalysisKind::Code) {
            analysis[range].fill(AnalysisKind::NotCode);
        }
    }

    // 割り込みベクタが逆アセンブル対象バンク内ならラベルを振る。
    if input.target_bank().contains_addr(ptr) {
        labels.set(ptr, Label::new(false));
    }

    // 割り込みハンドラのアドレスが取得できなければ終了。
    let Some((dst, bank_id)) = input.memory().fetch_addr(ptr) else {
        return;
    };

    // 割り込みハンドラのアドレスが既に NotCode とされていたら単に警告する。
    if analysis[dst] == AnalysisKind::NotCode {
        warn!("{name} handler address {dst:#06X} is NotCode");
        return;
    }

    // 割り込みハンドラのアドレスを Code とする。
    // また、これが逆アセンブル対象バンク内ならエントリポイントラベルも振る。
    analysis[dst] = AnalysisKind::Code;
    if input.target_bank_id() == bank_id {
        labels.set(dst, Label::new(true));
    }
}
