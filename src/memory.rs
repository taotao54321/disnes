use arrayvec::ArrayVec;
use thiserror::Error;

use crate::address::{Address, AddressRange, ArrayByAddress};
use crate::bank::Bank;
use crate::op::{Op, OpSucc, Opcode};

/// 論理アドレス空間。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    banks: Vec<Bank>,
    bank_ids: ArrayByAddress<Option<usize>>,
}

impl Memory {
    /// 指定したバンクたちがロードされた論理アドレス空間を作る。
    /// バンク間でアドレス範囲が重なっている場合、panic する。
    pub fn new(banks: Vec<Bank>) -> Self {
        use itertools::Itertools as _;

        for pair in banks.iter().combinations(2) {
            let (lhs, rhs) = (pair[0], pair[1]);
            let lhs_range = lhs.addr_range();
            let rhs_range = rhs.addr_range();
            assert!(
                !lhs_range.intersects(rhs_range),
                "bank range collision ({:#X}..={:#X} and {:#X}..={:#X})",
                lhs_range.min(),
                lhs_range.max(),
                rhs_range.min(),
                rhs_range.max()
            );
        }

        let mut bank_ids = ArrayByAddress::<Option<usize>>::default();
        for (id, bank) in banks.iter().enumerate() {
            bank_ids[bank.addr_range()].fill(Some(id));
        }

        Self { banks, bank_ids }
    }

    /// バンクリストを返す。
    pub fn banks(&self) -> &[Bank] {
        &self.banks
    }

    /// 指定したアドレスを含むバンクIDを返す。なければ `None` を返す。
    pub fn find_bank_id(&self, addr: Address) -> Option<usize> {
        self.bank_ids[addr]
    }

    /// 指定したアドレスの (内容, 所属バンクID) を返す。
    /// アドレスを含むバンクがなければ `None` を返す。
    pub fn get_byte(&self, addr: Address) -> Option<(u8, usize)> {
        self.find_bank_id(addr).map(|id| {
            let bank = &self.banks[id];
            let byte = bank.get_byte(addr).unwrap();
            (byte, id)
        })
    }

    /// 指定したアドレス範囲の (内容, 所属バンクID) を返す。
    /// アドレス範囲を完全に含むバンクがなければ `None` を返す。
    pub fn get_bytes(&self, range: AddressRange) -> Option<(&[u8], usize)> {
        self.find_bank_id(range.min()).and_then(|id| {
            let bank = &self.banks[id];
            bank.get_bytes(range).map(|buf| (buf, id))
        })
    }

    /// 指定したアドレス以降の ((空でない)内容, 所属バンクID) を返す。
    /// アドレスを含むバンクがなければ `None` を返す。
    pub fn get_bytes_from(&self, addr: Address) -> Option<(&[u8], usize)> {
        self.find_bank_id(addr).map(|id| {
            let bank = &self.banks[id];
            let buf = bank.get_bytes_from(addr).unwrap();
            (buf, id)
        })
    }

    /// 指定したアドレスから命令を読み取り、(命令, 所属バンクID) を返す。
    /// 完全な命令を読み取れなければ `FetchOpError` を返す。
    pub fn fetch_op(&self, addr: Address) -> Result<(Op, usize), FetchOpError> {
        let (buf, bank_id) = self.get_bytes_from(addr).ok_or(FetchOpError::Nothing)?;
        let opcode = Opcode::new(buf[0]);

        buf.get(..opcode.op_len().get())
            .map(|buf| {
                let op = Op::fetch(buf);
                (op, bank_id)
            })
            .ok_or_else(|| FetchOpError::Incomplete(buf.try_into().unwrap()))
    }

    /// 指定したアドレスからアドレス値(リトルエンディアン)を読み取り、(アドレス値, 所属バンクID) を返す。
    /// 完全なアドレス値を読み取れなければ `None` を返す。
    pub fn fetch_addr(&self, addr: Address) -> Option<(Address, usize)> {
        let (buf, bank_id) = self.get_bytes_from(addr)?;

        buf.get(..2).map(|buf| {
            let buf: [u8; 2] = buf.try_into().unwrap();
            let dst = Address::from_le_bytes(buf);
            (dst, bank_id)
        })
    }

    pub fn resolve_op_succ(&self, addr: Address, succ: OpSucc) -> OpSuccResolved {
        match succ {
            OpSucc::Normal(offset) => OpSuccResolved::Normal(addr.wrapping_add_unsigned(offset)),
            OpSucc::Brk => OpSuccResolved::Brk(self.fetch_addr(Address::new(0xFFFE)).map(|x| x.0)),
            OpSucc::Kil => OpSuccResolved::Kil,
            OpSucc::Branch(rel) => {
                let not_taken = addr.wrapping_add_unsigned(2_usize);
                let taken = not_taken.wrapping_add_signed(rel);
                OpSuccResolved::Branch { taken, not_taken }
            }
            OpSucc::Jsr(dst) => OpSuccResolved::Jsr(dst),
            OpSucc::Rti => OpSuccResolved::Rti,
            OpSucc::Rts => OpSuccResolved::Rts,
            OpSucc::JmpAbs(dst) => OpSuccResolved::JmpAbs(dst),
            OpSucc::JmpInd(ptr) => {
                // ページをまたぐポインタは読めないものとする。
                let dst = if (ptr.get() & 0xFF) == 0xFF {
                    None
                } else {
                    self.fetch_addr(ptr).map(|x| x.0)
                };
                OpSuccResolved::JmpInd(dst)
            }
        }
    }
}

/// `Memory::fetch_op` が返すエラー。
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum FetchOpError {
    /// オペコード自体が読み取れない。
    #[error("can't fetch opcode")]
    Nothing,
    /// オペコードは読み取れるが、命令が尻切れになっている。
    #[error("incomplete op: {0:?}")]
    Incomplete(ArrayVec<u8, 2>),
}

/// あるアドレス上の命令を実行したときの具体的なプログラムカウンタの値の候補。
/// アドレス空間内で wrap したり、別バンクへ移動するケースも含むことに注意。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpSuccResolved {
    /// 非制御フロー命令。値は現在の命令の直後のアドレス。
    Normal(Address),
    /// brk 命令。値は(取得できるなら) IRQ 割り込みハンドラのアドレス。
    Brk(Option<Address>),
    /// 全ての kil 命令。CPU が動作を停止する。
    Kil,
    /// 全ての分岐命令。分岐時/非分岐時それぞれのアドレスを保持する。
    Branch { taken: Address, not_taken: Address },
    /// jsr 命令。
    Jsr(Address),
    /// rti 命令。
    Rti,
    /// rts 命令。
    Rts,
    /// 絶対 jmp 命令。
    JmpAbs(Address),
    /// 間接 jmp 命令。値は(取得できるなら)ポインタの指す先。
    JmpInd(Option<Address>),
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    fn make_bank(addr: u16, body: impl Into<Vec<u8>>) -> Bank {
        Bank::new(Address::new(addr), body, false)
    }

    #[test]
    fn test_memory() {
        let memory = Memory::new(vec![
            make_bank(0x8000, [0, 1, 2]),
            make_bank(0x8003, [3, 4, 5]),
            make_bank(0x8007, [7, 8, 9]),
        ]);

        let find_bank_id = |addr: u16| memory.find_bank_id(Address::new(addr));
        let get_byte = |addr: u16| memory.get_byte(Address::new(addr));
        let get_bytes = |addr: u16, len: usize| {
            memory.get_bytes(AddressRange::from_start_len(
                Address::new(addr),
                NonZeroUsize::new(len).unwrap(),
            ))
        };
        let get_bytes_from = |addr: u16| memory.get_bytes_from(Address::new(addr));

        assert_eq!(find_bank_id(0x7FFF), None);
        assert_eq!(find_bank_id(0x8000), Some(0));
        assert_eq!(find_bank_id(0x8002), Some(0));
        assert_eq!(find_bank_id(0x8003), Some(1));
        assert_eq!(find_bank_id(0x8007), Some(2));
        assert_eq!(find_bank_id(0x800A), None);

        assert_eq!(get_byte(0x7FFF), None);
        assert_eq!(get_byte(0x8000), Some((0, 0)));
        assert_eq!(get_byte(0x8002), Some((2, 0)));
        assert_eq!(get_byte(0x8003), Some((3, 1)));
        assert_eq!(get_byte(0x8007), Some((7, 2)));
        assert_eq!(get_byte(0x800A), None);

        assert_eq!(get_bytes(0x7FFF, 3), None);
        assert_eq!(get_bytes(0x8000, 3), Some(([0, 1, 2].as_ref(), 0)));
        assert_eq!(get_bytes(0x8000, 4), None);
        assert_eq!(get_bytes(0x8003, 3), Some(([3, 4, 5].as_ref(), 1)));
        assert_eq!(get_bytes(0x8003, 4), None);
        assert_eq!(get_bytes(0x8007, 3), Some(([7, 8, 9].as_ref(), 2)));
        assert_eq!(get_bytes(0x8007, 4), None);

        assert_eq!(get_bytes_from(0x7FFF), None);
        assert_eq!(get_bytes_from(0x8000), Some(([0, 1, 2].as_ref(), 0)));
        assert_eq!(get_bytes_from(0x8001), Some(([1, 2].as_ref(), 0)));
        assert_eq!(get_bytes_from(0x8003), Some(([3, 4, 5].as_ref(), 1)));
        assert_eq!(get_bytes_from(0x8005), Some(([5].as_ref(), 1)));
        assert_eq!(get_bytes_from(0x8007), Some(([7, 8, 9].as_ref(), 2)));
        assert_eq!(get_bytes_from(0x800A), None);
    }

    #[test]
    fn test_memory_fetch_op() {
        let memory = Memory::new(vec![make_bank(0x8000, [0xA9, 0xFF, 0x20, 0xFF])]);

        let fetch_op = |addr: u16| memory.fetch_op(Address::new(addr));

        assert_eq!(fetch_op(0x7FFF), Err(FetchOpError::Nothing));
        assert_eq!(fetch_op(0x8000), Ok((Op::LdaImm(0xFF), 0)));
        assert_eq!(
            fetch_op(0x8002),
            Err(FetchOpError::Incomplete([0x20, 0xFF].into()))
        );
    }

    #[test]
    fn test_memory_fetch_addr() {
        let memory = Memory::new(vec![make_bank(0x8000, [0x12, 0x34, 0x56])]);

        let fetch_addr = |addr: u16| memory.fetch_addr(Address::new(addr));

        assert_eq!(fetch_addr(0x7FFF), None);
        assert_eq!(fetch_addr(0x8000), Some((Address::new(0x3412), 0)));
        assert_eq!(fetch_addr(0x8002), None);
    }

    #[test]
    #[should_panic]
    fn test_memory_bank_collision() {
        let _ = Memory::new(vec![
            make_bank(0x8000, [0, 1, 2]),
            make_bank(0x8002, [2, 3, 4]),
        ]);
    }
}
