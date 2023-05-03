use std::num::NonZeroUsize;

use anyhow::{bail, ensure};
use arrayvec::ArrayVec;

use crate::address::{Address, AddressRange};
use crate::op::Op;

/// アセンブリ全体。
///
/// 0 バイトではないことが保証される。
#[derive(Debug)]
pub struct Assembly {
    bank_addr_range: AddressRange,
    bank_name: String,
    statements: Vec<Statement>,
    labels: Labels,
}

impl Assembly {
    /// バンクのアドレス範囲を返す。
    pub fn bank_addr_range(&self) -> AddressRange {
        self.bank_addr_range
    }

    /// バンクの開始アドレスを返す。
    pub fn bank_addr(&self) -> Address {
        self.bank_addr_range.min()
    }

    /// バンク名を返す。
    pub fn bank_name(&self) -> &str {
        &self.bank_name
    }

    /// 文たちを返す。
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    /// `Labels` を返す。
    pub fn labels(&self) -> &Labels {
        &self.labels
    }
}

#[derive(Debug, Default)]
pub struct AssemblyBuilder {
    bank_addr_range: Option<AddressRange>,
    bank_name: Option<String>,
    statements: Option<Vec<Statement>>,
    labels: Option<Labels>,
}

impl AssemblyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> anyhow::Result<Assembly> {
        let Some(bank_addr_range) = self.bank_addr_range else {
            bail!("AssemblyBuilder: bank_addr_range is none");
        };
        let Some(bank_name) = self.bank_name else {
            bail!("AssemblyBuilder: bank_name is none");
        };
        let Some(statements) = self.statements else {
            bail!("AssemblyBuilder: statements is none");
        };
        let Some(labels) = self.labels else {
            bail!("AssemblyBuilder: labels is none");
        };

        ensure!(!statements.is_empty(), "AssemblyBuilder: 0 byte assembly");

        {
            let stmts_len_sum =
                NonZeroUsize::new(statements.iter().map(|stmt| stmt.len().get()).sum()).unwrap();
            ensure!(
                stmts_len_sum == bank_addr_range.len(),
                "AssemblyBuilder: bank address range mismatch"
            );
        }

        Ok(Assembly {
            bank_addr_range,
            bank_name,
            statements,
            labels,
        })
    }

    pub fn bank_addr_range(mut self, bank_addr_range: AddressRange) -> Self {
        self.bank_addr_range = Some(bank_addr_range);
        self
    }

    pub fn bank_name(mut self, bank_name: impl Into<String>) -> Self {
        self.bank_name = Some(bank_name.into());
        self
    }

    pub fn statements(mut self, statements: impl Into<Vec<Statement>>) -> Self {
        self.statements = Some(statements.into());
        self
    }

    pub fn labels(mut self, labels: Labels) -> Self {
        self.labels = Some(labels);
        self
    }
}

/// アセンブリの文。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Op(Op),
    /// NOTE: 中身は空であってはならない。
    IncompleteOp(ArrayVec<u8, 2>),
    Byte(u8),
}

impl Statement {
    /// バイト数を返す。
    pub fn len(&self) -> NonZeroUsize {
        match self {
            Self::Op(op) => op.len(),
            Self::IncompleteOp(buf) => NonZeroUsize::new(buf.len()).unwrap(),
            Self::Byte(_) => NonZeroUsize::new(1).unwrap(),
        }
    }
}

/// 論理アドレス空間上のラベルたち。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Labels(Box<[Option<Label>; 0x10000]>);

impl Default for Labels {
    fn default() -> Self {
        let inner: Box<[Option<Label>; 0x10000]> = vec![None; 0x10000].try_into().unwrap();

        Self(inner)
    }
}

impl Labels {
    pub fn get(&self, addr: Address) -> Option<&Label> {
        self.0[usize::from(addr)].as_ref()
    }

    /// 指定したアドレスにラベルを振る。
    ///
    /// 元々ラベルが振られていた場合、エントリポイントラベルを優先する。
    pub fn set(&mut self, addr: Address, label: Label) {
        let new = if let Some(orig) = self.0[usize::from(addr)].take() {
            Label::new(orig.is_entrypoint() || label.is_entrypoint())
        } else {
            label
        };

        self.0[usize::from(addr)] = Some(new);
    }
}

/// ラベル。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label {
    entrypoint: bool,
}

impl Label {
    pub fn new(entrypoint: bool) -> Self {
        Self { entrypoint }
    }

    /// ラベルがルーチンのエントリポイントかどうかを返す。
    pub fn is_entrypoint(&self) -> bool {
        self.entrypoint
    }
}
