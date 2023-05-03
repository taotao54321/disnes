use std::num::NonZeroUsize;
use std::path::PathBuf;

use anyhow::{bail, Context as _};
use itertools::Itertools as _;
use serde::{de::Error as _, Deserialize, Deserializer};

use crate::address::{Address, AddressRange};
use crate::bank::Bank;
use crate::cdl::{Cdl, CdlElement};
use crate::config::Config;
use crate::input::{Input, InputBuilder};
use crate::memory::Memory;
use crate::permission::{Permission, Permissions};
use crate::util;

/// TOML ファイルから読み込まれる構成。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(rename = "memory")]
    memory_regions: MemoryRegions,

    #[serde(rename = "banks")]
    bank_descs: BankDescs,

    config: Config,
}

impl Manifest {
    pub fn from_toml(s: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(toml::from_str(s.as_ref())?)
    }

    /// 逆アセンブル対象のバンク名を指定して `Input` と `Config` を作る。
    pub fn into_input_config(
        self,
        target_bank_name: impl Into<String>,
    ) -> anyhow::Result<(Input, Config)> {
        self._into_input_config(target_bank_name.into())
    }

    fn _into_input_config(self, target_bank_name: String) -> anyhow::Result<(Input, Config)> {
        // アドレス空間全体のパーミッションを設定。
        let mut perms = Permissions::default();
        for mr in self.memory_regions.0.iter() {
            let perm = Permission::new(mr.readable, mr.writable, mr.executable);
            perms[mr.addr_range()].fill(perm);
        }

        // バンクリストおよび CDL を作成。
        let mut banks = Vec::<Bank>::with_capacity(self.bank_descs.0.len());
        let mut cdl = Cdl::default();
        for bd in self.bank_descs.0.iter() {
            // 逆アセンブル対象または固定バンクのみロードする。
            if !(bd.name == target_bank_name || bd.fixed) {
                continue;
            }

            let body =
                util::fs_read_range(&bd.file, bd.file_offset, bd.len.get()).with_context(|| {
                    format!(
                        "can't read bank from '{}' (offset={:#X}, len={:#X})",
                        bd.file.display(),
                        bd.file_offset,
                        bd.len
                    )
                })?;
            let bank = Bank::new(bd.start, body, bd.fixed);
            banks.push(bank);

            if let Some(cdl_path) = bd.cdl.as_ref() {
                let cdl_body = util::fs_read_range(cdl_path, bd.cdl_offset, bd.len.get())
                    .with_context(|| {
                        format!(
                            "can't read CDL from '{}' (offset={:#X}, len={:#X})",
                            cdl_path.display(),
                            bd.cdl_offset,
                            bd.len
                        )
                    })?;
                let cdl_body: Vec<_> = cdl_body.into_iter().map(CdlElement::new).collect();
                cdl[bd.addr_range()].copy_from_slice(&cdl_body);
            }
        }

        // 逆アセンブル対象バンクのアドレスを取得。
        let target_bank_addr = self
            .bank_descs
            .0
            .iter()
            .find_map(|bd| (bd.name == target_bank_name).then_some(bd.start));
        let Some(target_bank_addr) = target_bank_addr else {
            bail!("target bank '{target_bank_name}' not found");
        };

        let memory = Memory::new(banks);

        let input = InputBuilder::new()
            .memory(memory)
            .permissions(perms)
            .cdl(cdl)
            .target_bank_addr(target_bank_addr)
            .target_bank_name(target_bank_name)
            .build()?;

        Ok((input, self.config))
    }
}

/// 各メモリ領域の構成。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, remote = "Self")]
struct MemoryRegions(Vec<MemoryRegion>);

impl<'de> Deserialize<'de> for MemoryRegions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // 各メモリ領域は互いに重なっていてはならない。
        for pair in this.0.iter().enumerate().combinations(2) {
            let ((lhs_id, lhs), (rhs_id, rhs)) = (pair[0], pair[1]);

            if lhs.addr_range().intersects(rhs.addr_range()) {
                return Err(D::Error::custom(format!(
                    "memory region {lhs_id} (start={:#X}, len={:#X}) intersects with memory region {rhs_id} (start={:#X}, len={:#X})",
                    lhs.start, lhs.len, rhs.start, rhs.len
                )));
            }
        }

        Ok(this)
    }
}

/// 1 つのメモリ領域の構成。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, remote = "Self")]
struct MemoryRegion {
    /// 開始アドレス。
    #[serde(deserialize_with = "deserialize_addr")]
    start: Address,

    /// バイト数。
    len: NonZeroUsize,

    /// 読み取り可能か。デフォルトは `false`。
    #[serde(default)]
    readable: bool,

    /// 書き込み可能か。デフォルトは `false`。
    #[serde(default)]
    writable: bool,

    /// 実行可能か。デフォルトは `false`。
    #[serde(default)]
    executable: bool,
}

impl<'de> Deserialize<'de> for MemoryRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // メモリ領域は論理アドレス空間に収まらなければならない。
        let addr_max = this.start.checked_add_unsigned(this.len.get() - 1);
        if addr_max.is_none() {
            return Err(D::Error::custom(format!(
                "memory region overflows (start={:#X}, len={:#X})",
                this.start, this.len
            )));
        }

        Ok(this)
    }
}

impl MemoryRegion {
    fn addr_range(&self) -> AddressRange {
        AddressRange::from_start_len(self.start, self.len)
    }
}

/// 各バンクの構成。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, remote = "Self")]
struct BankDescs(Vec<BankDesc>);

impl<'de> Deserialize<'de> for BankDescs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // 各バンクは一意な名前を持たねばならない。
        // また、固定バンクの場合は他のバンクと重なっていてはならない。
        for pair in this.0.iter().combinations(2) {
            let (lhs, rhs) = (pair[0], pair[1]);

            if lhs.name == rhs.name {
                return Err(D::Error::custom(format!(
                    "duplicated bank name: '{}'",
                    lhs.name
                )));
            }

            if (lhs.fixed || rhs.fixed) && lhs.addr_range().intersects(rhs.addr_range()) {
                return Err(D::Error::custom(format!(
                    "fixed bank must not intersect with another bank: bank '{}' and '{}'",
                    lhs.name, rhs.name
                )));
            }
        }

        Ok(this)
    }
}

/// 1 つのバンクの構成。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, remote = "Self")]
struct BankDesc {
    /// バンク名。
    name: String,

    /// 開始アドレス。
    #[serde(deserialize_with = "deserialize_addr")]
    start: Address,

    /// バイト数。
    len: NonZeroUsize,

    /// バンクの内容を保持するファイルのパス。
    file: PathBuf,

    /// バンクの内容の `file` 内オフセット。デフォルトは `0`。
    #[serde(default)]
    file_offset: usize,

    /// CDL ファイルのパス。
    cdl: Option<PathBuf>,

    /// CDL の `cdl` 内オフセット。デフォルトは `0`。
    #[serde(default)]
    cdl_offset: usize,

    /// 固定バンクかどうか。デフォルトは `false`。
    ///
    /// `true` の場合、他のバンクを逆アセンブルする際にもロードされ、解析に利用される。
    #[serde(default)]
    fixed: bool,
}

impl<'de> Deserialize<'de> for BankDesc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // バンクは論理アドレス空間に収まらなければならない。
        let addr_max = this.start.checked_add_unsigned(this.len.get() - 1);
        if addr_max.is_none() {
            return Err(D::Error::custom(format!(
                "bank '{} overflows (start={:#X}, len={:#X})",
                this.name, this.start, this.len
            )));
        }

        Ok(this)
    }
}

impl BankDesc {
    fn addr_range(&self) -> AddressRange {
        AddressRange::from_start_len(self.start, self.len)
    }
}

fn deserialize_addr<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let addr = u16::deserialize(deserializer)?;
    let addr = Address::new(addr);
    Ok(addr)
}
