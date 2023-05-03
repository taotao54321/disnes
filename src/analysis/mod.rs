mod cdl;
mod flow;
mod interrupt;
mod label;
mod linear_sweep;
mod op;
mod permission;

use crate::address::ArrayByAddress;
use crate::assembly::{Assembly, AssemblyBuilder, Labels};
use crate::config::AnalysisConfig;
use crate::input::Input;

/// 各種解析を行い、コード/非コードの識別とラベル振りを行い、`Assembly` を返す。
pub fn analyze(input: &Input, config: &AnalysisConfig) -> Assembly {
    let mut analysis = Analysis::default();
    let mut labels = Labels::default();

    self::cdl::analyze(&mut analysis, &mut labels, input);
    self::permission::analyze(&mut analysis, input);
    self::interrupt::analyze(&mut analysis, &mut labels, input, config);
    self::op::analyze(&mut analysis, input, config);
    self::flow::analyze(&mut analysis, input);
    let stmts = self::linear_sweep::analyze(&mut analysis, &mut labels, input);
    self::label::analyze(&analysis, &mut labels, input);

    let asm = AssemblyBuilder::new()
        .bank_addr_range(input.target_bank().addr_range())
        .bank_name(input.target_bank_name())
        .statements(stmts)
        .labels(labels)
        .build()
        .expect("AssemblyBuilder::build() should success");

    asm
}

/// 論理アドレス空間全体の解析結果。
type Analysis = ArrayByAddress<AnalysisKind>;

/// ある論理アドレスに対する解析結果。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AnalysisKind {
    Unknown,
    Code,
    NotCode,
}

impl Default for AnalysisKind {
    fn default() -> Self {
        Self::Unknown
    }
}
