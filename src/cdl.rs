//! [改造版 Mesen](https://github.com/taotao54321/Mesen) の CDL。

use crate::address::{Address, AddressRange};

/// 論理アドレス空間に対する CDL。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cdl(Box<[CdlElement; 0x10000]>);

impl Default for Cdl {
    fn default() -> Self {
        let inner: Box<[CdlElement; 0x10000]> =
            vec![CdlElement::default(); 0x10000].try_into().unwrap();

        Self(inner)
    }
}

impl Cdl {
    pub fn is_indirect_data_start(&self, addr: Address) -> bool {
        self[addr].is_indirect_data()
            && addr
                .checked_add_signed(-1_isize)
                .map_or(true, |addr_pre| !self[addr_pre].is_indirect_data())
    }

    pub fn is_pcm_data_start(&self, addr: Address) -> bool {
        self[addr].is_pcm_data()
            && addr
                .checked_add_signed(-1_isize)
                .map_or(true, |addr_pre| !self[addr_pre].is_pcm_data())
    }

    pub fn is_opcode(&self, addr: Address) -> bool {
        self[addr].is_opcode()
    }

    pub fn is_data(&self, addr: Address) -> bool {
        self[addr].is_data()
    }

    pub fn is_operand(&self, addr: Address) -> bool {
        self[addr].is_operand()
    }

    pub fn is_jump_target(&self, addr: Address) -> bool {
        self[addr].is_jump_target()
    }

    pub fn is_indirect_data(&self, addr: Address) -> bool {
        self[addr].is_indirect_data()
    }

    pub fn is_pcm_data(&self, addr: Address) -> bool {
        self[addr].is_pcm_data()
    }

    pub fn is_entrypoint(&self, addr: Address) -> bool {
        self[addr].is_entrypoint()
    }
}

impl std::ops::Index<Address> for Cdl {
    type Output = CdlElement;

    fn index(&self, addr: Address) -> &Self::Output {
        &self.0[usize::from(addr)]
    }
}

impl std::ops::IndexMut<Address> for Cdl {
    fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
        &mut self.0[usize::from(addr)]
    }
}

impl std::ops::Index<AddressRange> for Cdl {
    type Output = [CdlElement];

    fn index(&self, range: AddressRange) -> &Self::Output {
        &self.0[usize::from(range.min())..=usize::from(range.max())]
    }
}

impl std::ops::IndexMut<AddressRange> for Cdl {
    fn index_mut(&mut self, range: AddressRange) -> &mut Self::Output {
        &mut self.0[usize::from(range.min())..=usize::from(range.max())]
    }
}

/// ある論理アドレスに対する CDL。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CdlElement(u8);

impl CdlElement {
    pub const fn new(inner: u8) -> Self {
        Self(inner)
    }

    pub const fn is_opcode(self) -> bool {
        (self.0 & (1 << 0)) != 0
    }

    pub const fn is_data(self) -> bool {
        (self.0 & (1 << 1)) != 0
    }

    pub const fn is_operand(self) -> bool {
        (self.0 & (1 << 2)) != 0
    }

    pub const fn is_jump_target(self) -> bool {
        (self.0 & (1 << 4)) != 0
    }

    pub const fn is_indirect_data(self) -> bool {
        (self.0 & (1 << 5)) != 0
    }

    pub const fn is_pcm_data(self) -> bool {
        (self.0 & (1 << 6)) != 0
    }

    pub const fn is_entrypoint(self) -> bool {
        (self.0 & (1 << 7)) != 0
    }
}
