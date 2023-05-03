use std::num::NonZeroUsize;

use crate::address::{Address, AddressRange};

/// 論理アドレス空間上にロードされるバンク。
///
/// 空でなく、アドレス空間内に収まることが保証される。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bank {
    addr: Address,
    body: Vec<u8>,
    fixed: bool,
}

impl Bank {
    /// (開始アドレス, 内容, 固定バンクかどうか) を指定してバンクを作る。
    /// 内容が空だったり、アドレス空間内に収まらなければ panic する。
    pub fn new(addr: Address, body: impl Into<Vec<u8>>, fixed: bool) -> Self {
        Self::_new(addr, body.into(), fixed)
    }

    fn _new(addr: Address, body: Vec<u8>, fixed: bool) -> Self {
        assert!(!body.is_empty(), "bank is empty");
        assert!(
            addr.checked_add_unsigned(body.len() - 1).is_some(),
            "bank overflow (addr={:#X}, len={:#X})",
            addr,
            body.len()
        );

        Self { addr, body, fixed }
    }

    /// 開始アドレスを返す。
    pub fn addr(&self) -> Address {
        self.addr
    }

    /// 全体の内容を返す。
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    /// バイト数を返す。
    pub fn len(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.body.len()).unwrap()
    }

    /// アドレス範囲を返す。
    pub fn addr_range(&self) -> AddressRange {
        AddressRange::from_start_len(self.addr, self.len())
    }

    /// バンクが指定したアドレスを含むかどうかを返す。
    pub fn contains_addr(&self, addr: Address) -> bool {
        self.addr_range().contains_addr(addr)
    }

    /// バンクが指定したアドレス範囲を完全に含むかどうかを返す。
    pub fn contains_range(&self, range: AddressRange) -> bool {
        self.addr_range().contains_range(range)
    }

    /// 指定したアドレスの内容を返す。アドレスがバンク範囲外なら `None` を返す。
    pub fn get_byte(&self, addr: Address) -> Option<u8> {
        self.contains_addr(addr)
            .then(|| self.body[usize::from(addr) - usize::from(self.addr)])
    }

    /// 指定したアドレス範囲の内容を返す。アドレス範囲がバンクからはみ出すなら `None` を返す。
    pub fn get_bytes(&self, range: AddressRange) -> Option<&[u8]> {
        self.addr_range().contains_range(range).then(|| {
            &self.body[usize::from(range.min()) - usize::from(self.addr)..][..range.len().get()]
        })
    }

    /// 指定したアドレス以降の(空でない)内容を返す。アドレスがバンク範囲外なら `None` を返す。
    pub fn get_bytes_from(&self, addr: Address) -> Option<&[u8]> {
        self.contains_addr(addr)
            .then(|| &self.body[usize::from(addr) - usize::from(self.addr)..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank() {
        let addr = Address::new(0x8000);
        let body = [1, 2, 3];
        let len = NonZeroUsize::new(body.len()).unwrap();
        let bank = Bank::new(addr, body, true);

        assert_eq!(bank.addr(), addr);
        assert_eq!(bank.body(), body);
        assert!(bank.is_fixed());
        assert_eq!(bank.len(), len);
        assert_eq!(bank.addr_range(), AddressRange::from_start_len(addr, len));

        let get_byte = |addr: u16| bank.get_byte(Address::new(addr));
        let get_bytes = |start: u16, len: usize| {
            bank.get_bytes(AddressRange::from_start_len(
                Address::new(start),
                NonZeroUsize::new(len).unwrap(),
            ))
        };

        assert_eq!(get_byte(0x7FFF), None);
        assert_eq!(get_byte(0x8000), Some(1));
        assert_eq!(get_byte(0x8001), Some(2));
        assert_eq!(get_byte(0x8002), Some(3));
        assert_eq!(get_byte(0x8003), None);

        assert_eq!(get_bytes(0x7FFF, 2), None);
        assert_eq!(get_bytes(0x8000, 2), Some([1, 2].as_ref()));
        assert_eq!(get_bytes(0x8001, 2), Some([2, 3].as_ref()));
        assert_eq!(get_bytes(0x8002, 2), None);
    }

    #[test]
    #[should_panic]
    fn test_bank_empty() {
        let _ = Bank::new(Address::new(0), [], false);
    }

    #[test]
    #[should_panic]
    fn test_bank_overflow() {
        let _ = Bank::new(Address::new(0xFFFF), [1, 2], false);
    }
}
