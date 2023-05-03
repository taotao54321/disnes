use serde::Deserialize;

/// 逆アセンブラの設定。
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    analysis: AnalysisConfig,
}

impl Config {
    pub fn analysis(&self) -> &AnalysisConfig {
        &self.analysis
    }
}

/// 解析に関する設定。
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct AnalysisConfig {
    /// 可能ならば NMI 割り込みアドレスを見るか。デフォルトは `true`。
    use_nmi: bool,

    /// 可能ならば RESET 割り込みアドレスを見るか。デフォルトは `true`。
    use_reset: bool,

    /// 可能ならば IRQ 割り込みアドレスを見るか。デフォルトは `true`。
    use_irq: bool,

    /// brk 命令を許可するか。デフォルトは `false`。
    allow_brk: bool,

    /// clv 命令を許可するか。デフォルトは `false`。
    allow_clv: bool,

    /// sed 命令を許可するか。デフォルトは `false`。
    allow_sed: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            use_nmi: true,
            use_reset: true,
            use_irq: true,
            allow_brk: false,
            allow_clv: false,
            allow_sed: false,
        }
    }
}

impl AnalysisConfig {
    pub fn use_nmi(&self) -> bool {
        self.use_nmi
    }

    pub fn use_reset(&self) -> bool {
        self.use_reset
    }

    pub fn use_irq(&self) -> bool {
        self.use_irq
    }

    pub fn allow_brk(&self) -> bool {
        self.allow_brk
    }

    pub fn allow_clv(&self) -> bool {
        self.allow_clv
    }

    pub fn allow_sed(&self) -> bool {
        self.allow_sed
    }
}
