use std::num::NonZeroUsize;
use std::ops::RangeInclusive;

use derive_more::{Binary, Display, LowerHex, Octal, UpperHex};

/// 論理アドレス。
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Binary, Display, LowerHex, Octal, UpperHex,
)]
pub struct Address(u16);

impl Address {
    pub const fn new(inner: u16) -> Self {
        Self(inner)
    }

    pub const fn get(self) -> u16 {
        self.0
    }

    /// アドレスがゼロページ内かどうかを返す。
    pub const fn is_zeropage(self) -> bool {
        self.0 <= 0xFF
    }

    /// アドレスに符号なし数を加算した結果を返す。オーバーフローするなら `None` を返す。
    pub fn checked_add_unsigned(self, offset: impl Into<usize>) -> Option<Self> {
        self._checked_add_unsigned(offset.into())
    }

    fn _checked_add_unsigned(self, offset: usize) -> Option<Self> {
        usize::from(self.0)
            .checked_add(offset)
            .and_then(|addr| u16::try_from(addr).ok())
            .map(Self::new)
    }

    /// アドレスに符号付き数を加算した結果を返す。オーバーフローするなら `None` を返す。
    pub fn checked_add_signed(self, rel: impl Into<isize>) -> Option<Self> {
        self._checked_add_signed(rel.into())
    }

    fn _checked_add_signed(self, rel: isize) -> Option<Self> {
        usize::from(self.0)
            .checked_add_signed(rel)
            .and_then(|addr| u16::try_from(addr).ok())
            .map(Self::new)
    }

    /// アドレスに符号なし数を加算した結果を返す。オーバーフローを許す。
    pub fn wrapping_add_unsigned(self, offset: impl Into<usize>) -> Self {
        self._wrapping_add_unsigned(offset.into())
    }

    fn _wrapping_add_unsigned(self, offset: usize) -> Self {
        Self::new(usize::from(self.0).wrapping_add(offset) as u16)
    }

    /// アドレスに符号付き数を加算した結果を返す。オーバーフローを許す。
    pub fn wrapping_add_signed(self, rel: impl Into<isize>) -> Self {
        self._wrapping_add_signed(rel.into())
    }

    fn _wrapping_add_signed(self, rel: isize) -> Self {
        Self::new(usize::from(self.0).wrapping_add_signed(rel) as u16)
    }

    /// リトルエンディアンのバイト列をアドレスに変換する。
    pub const fn from_le_bytes(buf: [u8; 2]) -> Self {
        Self::new(u16::from_le_bytes(buf))
    }

    /// アドレスをリトルエンディアンのバイト列に変換する。
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    /// 全アドレスを昇順に列挙する。
    pub fn all() -> AddressIterator {
        AddressIterator(0..=u16::MAX)
    }
}

impl From<Address> for usize {
    fn from(addr: Address) -> Self {
        Self::from(addr.0)
    }
}

/// ゼロページアドレス。
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Binary, Display, LowerHex, Octal, UpperHex,
)]
pub struct ZpAddress(u8);

impl ZpAddress {
    pub const fn new(inner: u8) -> Self {
        Self(inner)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    /// ゼロページアドレスに符号なし数を加算した結果を返す。オーバーフローするなら `None` を返す。
    pub fn checked_add_unsigned(self, offset: impl Into<usize>) -> Option<Self> {
        self._checked_add_unsigned(offset.into())
    }

    fn _checked_add_unsigned(self, offset: usize) -> Option<Self> {
        usize::from(self.0)
            .checked_add(offset)
            .and_then(|addr| u8::try_from(addr).ok())
            .map(Self::new)
    }

    /// ゼロページアドレスに符号付き数を加算した結果を返す。オーバーフローするなら `None` を返す。
    pub fn checked_add_signed(self, rel: impl Into<isize>) -> Option<Self> {
        self._checked_add_signed(rel.into())
    }

    fn _checked_add_signed(self, rel: isize) -> Option<Self> {
        usize::from(self.0)
            .checked_add_signed(rel)
            .and_then(|addr| u8::try_from(addr).ok())
            .map(Self::new)
    }

    /// ゼロページアドレスに符号なし数を加算した結果を返す。オーバーフローを許す。
    pub fn wrapping_add_unsigned(self, offset: impl Into<usize>) -> Self {
        self._wrapping_add_unsigned(offset.into())
    }

    fn _wrapping_add_unsigned(self, offset: usize) -> Self {
        Self::new(usize::from(self.0).wrapping_add(offset) as u8)
    }

    /// ゼロページアドレスに符号付き数を加算した結果を返す。オーバーフローを許す。
    pub fn wrapping_add_signed(self, rel: impl Into<isize>) -> Self {
        self._wrapping_add_signed(rel.into())
    }

    fn _wrapping_add_signed(self, rel: isize) -> Self {
        Self::new(usize::from(self.0).wrapping_add_signed(rel) as u8)
    }

    /// リトルエンディアンのバイト列をゼロページアドレスに変換する。
    pub const fn from_le_bytes(buf: [u8; 1]) -> Self {
        Self::new(u8::from_le_bytes(buf))
    }

    /// ゼロページアドレスをリトルエンディアンのバイト列に変換する。
    pub const fn to_le_bytes(self) -> [u8; 1] {
        self.0.to_le_bytes()
    }

    /// 全ゼロページアドレスを昇順に列挙する。
    pub fn all() -> impl Iterator<Item = Self> {
        (0..=u8::MAX).map(Self::new)
    }
}

impl From<ZpAddress> for usize {
    fn from(zp: ZpAddress) -> Self {
        Self::from(zp.0)
    }
}

impl From<ZpAddress> for Address {
    fn from(zp: ZpAddress) -> Self {
        Self::new(u16::from(zp.0))
    }
}

/// 論理アドレスの範囲。
///
/// 範囲は空でなく、かつ `min <= max` であることが保証される。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressRange {
    min: Address,
    max: Address,
}

impl AddressRange {
    /// 開始アドレスと終了アドレス (inclusive) を指定して `AddressRange` を作る。
    /// `min > max` の場合、panic する。
    pub fn from_min_max(min: Address, max: Address) -> Self {
        assert!(min <= max);

        Self { min, max }
    }

    /// 開始アドレスと長さを指定して `AddressRange` を作る。
    /// オーバーフローする場合、panic する。
    pub fn from_start_len(min: Address, len: NonZeroUsize) -> Self {
        let max = min
            .checked_add_unsigned(len.get() - 1)
            .expect("address range overflow");

        Self { min, max }
    }

    pub fn min(self) -> Address {
        self.min
    }

    pub fn max(self) -> Address {
        self.max
    }

    pub fn len(self) -> NonZeroUsize {
        NonZeroUsize::new(usize::from(self.max.get() - self.min.get()) + 1).unwrap()
    }

    /// 2 つのアドレス範囲が共通部分を持つかどうかを返す。
    pub fn intersects(self, other: Self) -> bool {
        !(self.max < other.min || other.max < self.min)
    }

    /// このアドレス範囲が指定したアドレスを含むかどうかを返す。
    pub fn contains_addr(self, addr: Address) -> bool {
        self.min <= addr && addr <= self.max
    }

    /// 指定したアドレス範囲がこのアドレス範囲の部分集合かどうかを返す。
    pub fn contains_range(self, other: Self) -> bool {
        self.min <= other.min && other.max <= self.max
    }
}

impl IntoIterator for AddressRange {
    type Item = Address;
    type IntoIter = AddressIterator;

    /// 範囲内の全アドレスを昇順に列挙する。
    fn into_iter(self) -> AddressIterator {
        AddressIterator(self.min.0..=self.max.0)
    }
}

/// 論理アドレスを列挙するイテレーター。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressIterator(RangeInclusive<u16>);

impl Iterator for AddressIterator {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Address::new)
    }
}

/// `Address`, `AddressRange` でインデックスアクセスできる配列。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayByAddress<T>(Box<[T; 0x10000]>);

impl<T: Default> Default for ArrayByAddress<T> {
    fn default() -> Self {
        let inner: Box<[T; 0x10000]> = std::iter::repeat_with(|| T::default())
            .take(0x10000)
            .collect::<Box<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!("slice length should be 0x10000"));

        Self(inner)
    }
}

impl<T> std::ops::Index<Address> for ArrayByAddress<T> {
    type Output = T;

    fn index(&self, addr: Address) -> &Self::Output {
        &self.0[usize::from(addr)]
    }
}

impl<T> std::ops::IndexMut<Address> for ArrayByAddress<T> {
    fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
        &mut self.0[usize::from(addr)]
    }
}

impl<T> std::ops::Index<AddressRange> for ArrayByAddress<T> {
    type Output = [T];

    fn index(&self, range: AddressRange) -> &Self::Output {
        &self.0[usize::from(range.min)..=usize::from(range.max)]
    }
}

impl<T> std::ops::IndexMut<AddressRange> for ArrayByAddress<T> {
    fn index_mut(&mut self, range: AddressRange) -> &mut Self::Output {
        &mut self.0[usize::from(range.min)..=usize::from(range.max)]
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_address_checked_add_unsigned() {
        fn f(addr: u16, offset: usize) -> Option<Address> {
            Address::new(addr).checked_add_unsigned(offset)
        }

        assert_eq!(f(0, 1), Some(Address::new(1)));
        assert_eq!(f(0xFFFF, 1), None);
    }

    #[test]
    fn test_address_checked_add_signed() {
        fn f(addr: u16, rel: isize) -> Option<Address> {
            Address::new(addr).checked_add_signed(rel)
        }

        assert_eq!(f(0, 1), Some(Address::new(1)));
        assert_eq!(f(0xFFFF, 1), None);
        assert_eq!(f(0, -1), None);
        assert_eq!(f(0xFFFF, -1), Some(Address::new(0xFFFE)));
    }

    #[test]
    fn test_address_wrapping_add_unsigned() {
        fn f(addr: u16, offset: usize) -> Address {
            Address::new(addr).wrapping_add_unsigned(offset)
        }

        assert_eq!(f(0, 1), Address::new(1));
        assert_eq!(f(0xFFFF, 1), Address::new(0));
    }

    #[test]
    fn test_address_wrapping_add_signed() {
        fn f(addr: u16, rel: isize) -> Address {
            Address::new(addr).wrapping_add_signed(rel)
        }

        assert_eq!(f(0, 1), Address::new(1));
        assert_eq!(f(0xFFFF, 1), Address::new(0));
        assert_eq!(f(0, -1), Address::new(0xFFFF));
        assert_eq!(f(0xFFFF, -1), Address::new(0xFFFE));
    }

    #[test]
    fn test_zp_address_checked_add_unsigned() {
        fn f(addr: u8, offset: usize) -> Option<ZpAddress> {
            ZpAddress::new(addr).checked_add_unsigned(offset)
        }

        assert_eq!(f(0, 1), Some(ZpAddress::new(1)));
        assert_eq!(f(0xFF, 1), None);
    }

    #[test]
    fn test_zp_address_checked_add_signed() {
        fn f(addr: u8, rel: isize) -> Option<ZpAddress> {
            ZpAddress::new(addr).checked_add_signed(rel)
        }

        assert_eq!(f(0, 1), Some(ZpAddress::new(1)));
        assert_eq!(f(0xFF, 1), None);
        assert_eq!(f(0, -1), None);
        assert_eq!(f(0xFF, -1), Some(ZpAddress::new(0xFE)));
    }

    #[test]
    fn test_zp_address_wrapping_add_unsigned() {
        fn f(addr: u8, offset: usize) -> ZpAddress {
            ZpAddress::new(addr).wrapping_add_unsigned(offset)
        }

        assert_eq!(f(0, 1), ZpAddress::new(1));
        assert_eq!(f(0xFF, 1), ZpAddress::new(0));
    }

    #[test]
    fn test_zp_address_wrapping_add_signed() {
        fn f(addr: u8, rel: isize) -> ZpAddress {
            ZpAddress::new(addr).wrapping_add_signed(rel)
        }

        assert_eq!(f(0, 1), ZpAddress::new(1));
        assert_eq!(f(0xFF, 1), ZpAddress::new(0));
        assert_eq!(f(0, -1), ZpAddress::new(0xFF));
        assert_eq!(f(0xFF, -1), ZpAddress::new(0xFE));
    }

    fn make_range(min: u16, len: usize) -> AddressRange {
        AddressRange::from_start_len(Address::new(min), NonZeroUsize::new(len).unwrap())
    }

    #[test]
    fn test_address_range() {
        {
            let range = make_range(0, 1);
            assert_eq!(range.min(), Address::new(0));
            assert_eq!(range.max(), Address::new(0));
        }
        {
            let range = make_range(0x8000, 0x8000);
            assert_eq!(range.min(), Address::new(0x8000));
            assert_eq!(range.max(), Address::new(0xFFFF));
        }
    }

    #[test]
    #[should_panic]
    fn test_address_range_overflow() {
        let _ = make_range(0xFFFF, 2);
    }

    #[test]
    fn test_address_range_intersects() {
        fn f((min1, len1): (u16, usize), (min2, len2): (u16, usize)) -> bool {
            let range1 = make_range(min1, len1);
            let range2 = make_range(min2, len2);
            range1.intersects(range2)
        }

        assert!(f((0, 1), (0, 1)));
        assert!(f((0, 10), (3, 3)));
        assert!(f((3, 3), (0, 10)));
        assert!(f((0, 10), (9, 3)));
        assert!(f((9, 3), (0, 10)));

        assert!(!f((0, 10), (10, 10)));
        assert!(!f((10, 10), (0, 10)));
    }

    #[test]
    fn test_address_range_contains_addr() {
        fn f((min, len): (u16, usize), addr: u16) -> bool {
            let range = make_range(min, len);
            let addr = Address::new(addr);
            range.contains_addr(addr)
        }

        assert!(f((0, 1), 0));
        assert!(f((0, 10), 5));

        assert!(!f((10, 10), 9));
        assert!(!f((10, 10), 20));
    }

    #[test]
    fn test_address_range_contains_range() {
        fn f((min1, len1): (u16, usize), (min2, len2): (u16, usize)) -> bool {
            let range1 = make_range(min1, len1);
            let range2 = make_range(min2, len2);
            range1.contains_range(range2)
        }

        assert!(f((0, 1), (0, 1)));
        assert!(f((10, 10), (13, 3)));
        assert!(f((10, 10), (10, 3)));
        assert!(f((10, 10), (17, 3)));

        assert!(!f((10, 10), (0, 10)));
        assert!(!f((10, 10), (9, 3)));
        assert!(!f((10, 10), (19, 3)));
        assert!(!f((10, 10), (20, 10)));
        assert!(!f((10, 10), (0, 30)));
    }

    #[test]
    fn test_address_range_into_iter() {
        fn f(min: u16, len: usize) -> AddressIterator {
            let range = make_range(min, len);
            range.into_iter()
        }

        fn addrs<const N: usize>(xs: [u16; N]) -> [Address; N] {
            xs.map(Address::new)
        }

        assert_equal(f(0, 1), addrs([0]));
        assert_equal(f(10, 5), addrs([10, 11, 12, 13, 14]));
    }

    #[test]
    fn test_array_by_address() {
        let mut ary = ArrayByAddress::<u8>::default();
        ary[Address::new(1)] = 1;
        ary[AddressRange::from_start_len(Address::new(2), NonZeroUsize::new(2).unwrap())]
            .copy_from_slice(&[2, 3]);

        assert_eq!(ary[Address::new(0)], 0);
        assert_eq!(ary[Address::new(1)], 1);
        assert_eq!(ary[Address::new(2)], 2);
        assert_eq!(ary[Address::new(3)], 3);
        assert_eq!(ary[Address::new(4)], 0);
        assert_eq!(
            ary[AddressRange::from_start_len(Address::new(0), NonZeroUsize::new(5).unwrap())],
            [0, 1, 2, 3, 0]
        );
    }
}
