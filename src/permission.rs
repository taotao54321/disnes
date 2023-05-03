use crate::address::ArrayByAddress;

/// 論理アドレス空間全体のパーミッション。
pub type Permissions = ArrayByAddress<Permission>;

/// ある論理アドレスに対するパーミッション。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Permission(u8);

impl Permission {
    const FLAG_R: u8 = 1 << 0;
    const FLAG_W: u8 = 1 << 1;
    const FLAG_X: u8 = 1 << 2;

    pub const fn new(readable: bool, writable: bool, executable: bool) -> Self {
        let r = if readable { Self::FLAG_R } else { 0 };
        let w = if writable { Self::FLAG_W } else { 0 };
        let x = if executable { Self::FLAG_X } else { 0 };

        Self(r | w | x)
    }

    pub const fn is_readable(self) -> bool {
        (self.0 & Self::FLAG_R) != 0
    }

    pub const fn is_writable(self) -> bool {
        (self.0 & Self::FLAG_W) != 0
    }

    pub const fn is_executable(self) -> bool {
        (self.0 & Self::FLAG_X) != 0
    }
}
