use anyhow::bail;

use crate::address::Address;
use crate::bank::Bank;
use crate::cdl::Cdl;
use crate::memory::Memory;
use crate::permission::Permissions;

/// 逆アセンブラに対する入力。
#[derive(Debug)]
pub struct Input {
    memory: Memory,
    permissions: Permissions,
    cdl: Cdl,
    target_bank_id: usize,
    target_bank_name: String,
}

impl Input {
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }

    pub fn cdl(&self) -> &Cdl {
        &self.cdl
    }

    pub fn target_bank_id(&self) -> usize {
        self.target_bank_id
    }

    pub fn target_bank_name(&self) -> &str {
        &self.target_bank_name
    }

    pub fn target_bank(&self) -> &Bank {
        &self.memory.banks()[self.target_bank_id]
    }
}

#[derive(Debug, Default)]
pub struct InputBuilder {
    memory: Option<Memory>,
    permissions: Option<Permissions>,
    cdl: Option<Cdl>,
    target_bank_addr: Option<Address>,
    target_bank_name: Option<String>,
}

impl InputBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> anyhow::Result<Input> {
        let Some(memory) = self.memory else {
            bail!("InputBuilder: memory is none");
        };
        let Some(permissions) = self.permissions else {
            bail!("InputBuilder: permissions is none");
        };
        let Some(cdl) = self.cdl else {
            bail!("InputBuilder: cdl is none");
        };
        let Some(target_bank_addr) = self.target_bank_addr else {
            bail!("InputBuilder: target_bank_addr is none");
        };
        let Some(target_bank_name) = self.target_bank_name else {
            bail!("InputBuilder: target_bank_name is none");
        };

        let Some(target_bank_id) = memory.find_bank_id(target_bank_addr) else {
            bail!("InputBuilder: target bank not found");
        };

        Ok(Input {
            memory,
            permissions,
            cdl,
            target_bank_id,
            target_bank_name,
        })
    }

    pub fn memory(mut self, memory: Memory) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    pub fn cdl(mut self, cdl: Cdl) -> Self {
        self.cdl = Some(cdl);
        self
    }

    pub fn target_bank_addr(mut self, target_bank_addr: Address) -> Self {
        self.target_bank_addr = Some(target_bank_addr);
        self
    }

    pub fn target_bank_name(mut self, target_bank_name: impl Into<String>) -> Self {
        self.target_bank_name = Some(target_bank_name.into());
        self
    }
}
