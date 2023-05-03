use std::num::NonZeroUsize;

use arrayvec::ArrayVec;

use crate::address::{Address, ZpAddress};

/// CPU 命令。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    // 0x00
    Brk,
    OraIndX(ZpAddress),
    Kil02,
    SloIndX(ZpAddress),
    NopZp04(ZpAddress),
    OraZp(ZpAddress),
    AslZp(ZpAddress),
    SloZp(ZpAddress),
    Php,
    OraImm(u8),
    AslAcc,
    Anc0B(u8),
    NopAbs(Address),
    OraAbs(Address),
    AslAbs(Address),
    SloAbs(Address),

    // 0x10
    Bpl(i8),
    OraIndY(ZpAddress),
    Kil12,
    SloIndY(ZpAddress),
    NopZpX14(ZpAddress),
    OraZpX(ZpAddress),
    AslZpX(ZpAddress),
    SloZpX(ZpAddress),
    Clc,
    OraAbsY(Address),
    Nop1A,
    SloAbsY(Address),
    NopAbsX1C(Address),
    OraAbsX(Address),
    AslAbsX(Address),
    SloAbsX(Address),

    // 0x20
    Jsr(Address),
    AndIndX(ZpAddress),
    Kil22,
    RlaIndX(ZpAddress),
    BitZp(ZpAddress),
    AndZp(ZpAddress),
    RolZp(ZpAddress),
    RlaZp(ZpAddress),
    Plp,
    AndImm(u8),
    RolAcc,
    Anc2B(u8),
    BitAbs(Address),
    AndAbs(Address),
    RolAbs(Address),
    RlaAbs(Address),

    // 0x30
    Bmi(i8),
    AndIndY(ZpAddress),
    Kil32,
    RlaIndY(ZpAddress),
    NopZpX34(ZpAddress),
    AndZpX(ZpAddress),
    RolZpX(ZpAddress),
    RlaZpX(ZpAddress),
    Sec,
    AndAbsY(Address),
    Nop3A,
    RlaAbsY(Address),
    NopAbsX3C(Address),
    AndAbsX(Address),
    RolAbsX(Address),
    RlaAbsX(Address),

    // 0x40
    Rti,
    EorIndX(ZpAddress),
    Kil42,
    SreIndX(ZpAddress),
    NopZp44(ZpAddress),
    EorZp(ZpAddress),
    LsrZp(ZpAddress),
    SreZp(ZpAddress),
    Pha,
    EorImm(u8),
    LsrAcc,
    Alr(u8),
    JmpAbs(Address),
    EorAbs(Address),
    LsrAbs(Address),
    SreAbs(Address),

    // 0x50
    Bvc(i8),
    EorIndY(ZpAddress),
    Kil52,
    SreIndY(ZpAddress),
    NopZpX54(ZpAddress),
    EorZpX(ZpAddress),
    LsrZpX(ZpAddress),
    SreZpX(ZpAddress),
    Cli,
    EorAbsY(Address),
    Nop5A,
    SreAbsY(Address),
    NopAbsX5C(Address),
    EorAbsX(Address),
    LsrAbsX(Address),
    SreAbsX(Address),

    // 0x60
    Rts,
    AdcIndX(ZpAddress),
    Kil62,
    RraIndX(ZpAddress),
    NopZp64(ZpAddress),
    AdcZp(ZpAddress),
    RorZp(ZpAddress),
    RraZp(ZpAddress),
    Pla,
    AdcImm(u8),
    RorAcc,
    Arr(u8),
    JmpInd(Address),
    AdcAbs(Address),
    RorAbs(Address),
    RraAbs(Address),

    // 0x70
    Bvs(i8),
    AdcIndY(ZpAddress),
    Kil72,
    RraIndY(ZpAddress),
    NopZpX74(ZpAddress),
    AdcZpX(ZpAddress),
    RorZpX(ZpAddress),
    RraZpX(ZpAddress),
    Sei,
    AdcAbsY(Address),
    Nop7A,
    RraAbsY(Address),
    NopAbsX7C(Address),
    AdcAbsX(Address),
    RorAbsX(Address),
    RraAbsX(Address),

    // 0x80
    NopImm80(u8),
    StaIndX(ZpAddress),
    NopImm82(u8),
    SaxIndX(ZpAddress),
    StyZp(ZpAddress),
    StaZp(ZpAddress),
    StxZp(ZpAddress),
    SaxZp(ZpAddress),
    Dey,
    NopImm89(u8),
    Txa,
    Xaa(u8),
    StyAbs(Address),
    StaAbs(Address),
    StxAbs(Address),
    SaxAbs(Address),

    // 0x90
    Bcc(i8),
    StaIndY(ZpAddress),
    Kil92,
    AhxIndY(ZpAddress),
    StyZpX(ZpAddress),
    StaZpX(ZpAddress),
    StxZpY(ZpAddress),
    SaxZpY(ZpAddress),
    Tya,
    StaAbsY(Address),
    Txs,
    Tas(Address),
    Shy(Address),
    StaAbsX(Address),
    Shx(Address),
    AhxAbsY(Address),

    // 0xA0
    LdyImm(u8),
    LdaIndX(ZpAddress),
    LdxImm(u8),
    LaxIndX(ZpAddress),
    LdyZp(ZpAddress),
    LdaZp(ZpAddress),
    LdxZp(ZpAddress),
    LaxZp(ZpAddress),
    Tay,
    LdaImm(u8),
    Tax,
    LaxImm(u8),
    LdyAbs(Address),
    LdaAbs(Address),
    LdxAbs(Address),
    LaxAbs(Address),

    // 0xB0
    Bcs(i8),
    LdaIndY(ZpAddress),
    KilB2,
    LaxIndY(ZpAddress),
    LdyZpX(ZpAddress),
    LdaZpX(ZpAddress),
    LdxZpY(ZpAddress),
    LaxZpY(ZpAddress),
    Clv,
    LdaAbsY(Address),
    Tsx,
    Las(Address),
    LdyAbsX(Address),
    LdaAbsX(Address),
    LdxAbsY(Address),
    LaxAbsY(Address),

    // 0xC0
    CpyImm(u8),
    CmpIndX(ZpAddress),
    NopImmC2(u8),
    DcpIndX(ZpAddress),
    CpyZp(ZpAddress),
    CmpZp(ZpAddress),
    DecZp(ZpAddress),
    DcpZp(ZpAddress),
    Iny,
    CmpImm(u8),
    Dex,
    Axs(u8),
    CpyAbs(Address),
    CmpAbs(Address),
    DecAbs(Address),
    DcpAbs(Address),

    // 0xD0
    Bne(i8),
    CmpIndY(ZpAddress),
    KilD2,
    DcpIndY(ZpAddress),
    NopZpXD4(ZpAddress),
    CmpZpX(ZpAddress),
    DecZpX(ZpAddress),
    DcpZpX(ZpAddress),
    Cld,
    CmpAbsY(Address),
    NopDA,
    DcpAbsY(Address),
    NopAbsXDC(Address),
    CmpAbsX(Address),
    DecAbsX(Address),
    DcpAbsX(Address),

    // 0xE0
    CpxImm(u8),
    SbcIndX(ZpAddress),
    NopImmE2(u8),
    IscIndX(ZpAddress),
    CpxZp(ZpAddress),
    SbcZp(ZpAddress),
    IncZp(ZpAddress),
    IscZp(ZpAddress),
    Inx,
    SbcImmE9(u8),
    NopEA,
    SbcImmEB(u8),
    CpxAbs(Address),
    SbcAbs(Address),
    IncAbs(Address),
    IscAbs(Address),

    // 0xF0
    Beq(i8),
    SbcIndY(ZpAddress),
    KilF2,
    IscIndY(ZpAddress),
    NopZpXF4(ZpAddress),
    SbcZpX(ZpAddress),
    IncZpX(ZpAddress),
    IscZpX(ZpAddress),
    Sed,
    SbcAbsY(Address),
    NopFA,
    IscAbsY(Address),
    NopAbsXFC(Address),
    SbcAbsX(Address),
    IncAbsX(Address),
    IscAbsX(Address),
}

impl Op {
    /// オペコードを返す。
    pub const fn opcode(self) -> Opcode {
        let inner = match self {
            Self::Brk => 0x00,
            Self::OraIndX(_) => 0x01,
            Self::Kil02 => 0x02,
            Self::SloIndX(_) => 0x03,
            Self::NopZp04(_) => 0x04,
            Self::OraZp(_) => 0x05,
            Self::AslZp(_) => 0x06,
            Self::SloZp(_) => 0x07,
            Self::Php => 0x08,
            Self::OraImm(_) => 0x09,
            Self::AslAcc => 0x0A,
            Self::Anc0B(_) => 0x0B,
            Self::NopAbs(_) => 0x0C,
            Self::OraAbs(_) => 0x0D,
            Self::AslAbs(_) => 0x0E,
            Self::SloAbs(_) => 0x0F,
            Self::Bpl(_) => 0x10,
            Self::OraIndY(_) => 0x11,
            Self::Kil12 => 0x12,
            Self::SloIndY(_) => 0x13,
            Self::NopZpX14(_) => 0x14,
            Self::OraZpX(_) => 0x15,
            Self::AslZpX(_) => 0x16,
            Self::SloZpX(_) => 0x17,
            Self::Clc => 0x18,
            Self::OraAbsY(_) => 0x19,
            Self::Nop1A => 0x1A,
            Self::SloAbsY(_) => 0x1B,
            Self::NopAbsX1C(_) => 0x1C,
            Self::OraAbsX(_) => 0x1D,
            Self::AslAbsX(_) => 0x1E,
            Self::SloAbsX(_) => 0x1F,
            Self::Jsr(_) => 0x20,
            Self::AndIndX(_) => 0x21,
            Self::Kil22 => 0x22,
            Self::RlaIndX(_) => 0x23,
            Self::BitZp(_) => 0x24,
            Self::AndZp(_) => 0x25,
            Self::RolZp(_) => 0x26,
            Self::RlaZp(_) => 0x27,
            Self::Plp => 0x28,
            Self::AndImm(_) => 0x29,
            Self::RolAcc => 0x2A,
            Self::Anc2B(_) => 0x2B,
            Self::BitAbs(_) => 0x2C,
            Self::AndAbs(_) => 0x2D,
            Self::RolAbs(_) => 0x2E,
            Self::RlaAbs(_) => 0x2F,
            Self::Bmi(_) => 0x30,
            Self::AndIndY(_) => 0x31,
            Self::Kil32 => 0x32,
            Self::RlaIndY(_) => 0x33,
            Self::NopZpX34(_) => 0x34,
            Self::AndZpX(_) => 0x35,
            Self::RolZpX(_) => 0x36,
            Self::RlaZpX(_) => 0x37,
            Self::Sec => 0x38,
            Self::AndAbsY(_) => 0x39,
            Self::Nop3A => 0x3A,
            Self::RlaAbsY(_) => 0x3B,
            Self::NopAbsX3C(_) => 0x3C,
            Self::AndAbsX(_) => 0x3D,
            Self::RolAbsX(_) => 0x3E,
            Self::RlaAbsX(_) => 0x3F,
            Self::Rti => 0x40,
            Self::EorIndX(_) => 0x41,
            Self::Kil42 => 0x42,
            Self::SreIndX(_) => 0x43,
            Self::NopZp44(_) => 0x44,
            Self::EorZp(_) => 0x45,
            Self::LsrZp(_) => 0x46,
            Self::SreZp(_) => 0x47,
            Self::Pha => 0x48,
            Self::EorImm(_) => 0x49,
            Self::LsrAcc => 0x4A,
            Self::Alr(_) => 0x4B,
            Self::JmpAbs(_) => 0x4C,
            Self::EorAbs(_) => 0x4D,
            Self::LsrAbs(_) => 0x4E,
            Self::SreAbs(_) => 0x4F,
            Self::Bvc(_) => 0x50,
            Self::EorIndY(_) => 0x51,
            Self::Kil52 => 0x52,
            Self::SreIndY(_) => 0x53,
            Self::NopZpX54(_) => 0x54,
            Self::EorZpX(_) => 0x55,
            Self::LsrZpX(_) => 0x56,
            Self::SreZpX(_) => 0x57,
            Self::Cli => 0x58,
            Self::EorAbsY(_) => 0x59,
            Self::Nop5A => 0x5A,
            Self::SreAbsY(_) => 0x5B,
            Self::NopAbsX5C(_) => 0x5C,
            Self::EorAbsX(_) => 0x5D,
            Self::LsrAbsX(_) => 0x5E,
            Self::SreAbsX(_) => 0x5F,
            Self::Rts => 0x60,
            Self::AdcIndX(_) => 0x61,
            Self::Kil62 => 0x62,
            Self::RraIndX(_) => 0x63,
            Self::NopZp64(_) => 0x64,
            Self::AdcZp(_) => 0x65,
            Self::RorZp(_) => 0x66,
            Self::RraZp(_) => 0x67,
            Self::Pla => 0x68,
            Self::AdcImm(_) => 0x69,
            Self::RorAcc => 0x6A,
            Self::Arr(_) => 0x6B,
            Self::JmpInd(_) => 0x6C,
            Self::AdcAbs(_) => 0x6D,
            Self::RorAbs(_) => 0x6E,
            Self::RraAbs(_) => 0x6F,
            Self::Bvs(_) => 0x70,
            Self::AdcIndY(_) => 0x71,
            Self::Kil72 => 0x72,
            Self::RraIndY(_) => 0x73,
            Self::NopZpX74(_) => 0x74,
            Self::AdcZpX(_) => 0x75,
            Self::RorZpX(_) => 0x76,
            Self::RraZpX(_) => 0x77,
            Self::Sei => 0x78,
            Self::AdcAbsY(_) => 0x79,
            Self::Nop7A => 0x7A,
            Self::RraAbsY(_) => 0x7B,
            Self::NopAbsX7C(_) => 0x7C,
            Self::AdcAbsX(_) => 0x7D,
            Self::RorAbsX(_) => 0x7E,
            Self::RraAbsX(_) => 0x7F,
            Self::NopImm80(_) => 0x80,
            Self::StaIndX(_) => 0x81,
            Self::NopImm82(_) => 0x82,
            Self::SaxIndX(_) => 0x83,
            Self::StyZp(_) => 0x84,
            Self::StaZp(_) => 0x85,
            Self::StxZp(_) => 0x86,
            Self::SaxZp(_) => 0x87,
            Self::Dey => 0x88,
            Self::NopImm89(_) => 0x89,
            Self::Txa => 0x8A,
            Self::Xaa(_) => 0x8B,
            Self::StyAbs(_) => 0x8C,
            Self::StaAbs(_) => 0x8D,
            Self::StxAbs(_) => 0x8E,
            Self::SaxAbs(_) => 0x8F,
            Self::Bcc(_) => 0x90,
            Self::StaIndY(_) => 0x91,
            Self::Kil92 => 0x92,
            Self::AhxIndY(_) => 0x93,
            Self::StyZpX(_) => 0x94,
            Self::StaZpX(_) => 0x95,
            Self::StxZpY(_) => 0x96,
            Self::SaxZpY(_) => 0x97,
            Self::Tya => 0x98,
            Self::StaAbsY(_) => 0x99,
            Self::Txs => 0x9A,
            Self::Tas(_) => 0x9B,
            Self::Shy(_) => 0x9C,
            Self::StaAbsX(_) => 0x9D,
            Self::Shx(_) => 0x9E,
            Self::AhxAbsY(_) => 0x9F,
            Self::LdyImm(_) => 0xA0,
            Self::LdaIndX(_) => 0xA1,
            Self::LdxImm(_) => 0xA2,
            Self::LaxIndX(_) => 0xA3,
            Self::LdyZp(_) => 0xA4,
            Self::LdaZp(_) => 0xA5,
            Self::LdxZp(_) => 0xA6,
            Self::LaxZp(_) => 0xA7,
            Self::Tay => 0xA8,
            Self::LdaImm(_) => 0xA9,
            Self::Tax => 0xAA,
            Self::LaxImm(_) => 0xAB,
            Self::LdyAbs(_) => 0xAC,
            Self::LdaAbs(_) => 0xAD,
            Self::LdxAbs(_) => 0xAE,
            Self::LaxAbs(_) => 0xAF,
            Self::Bcs(_) => 0xB0,
            Self::LdaIndY(_) => 0xB1,
            Self::KilB2 => 0xB2,
            Self::LaxIndY(_) => 0xB3,
            Self::LdyZpX(_) => 0xB4,
            Self::LdaZpX(_) => 0xB5,
            Self::LdxZpY(_) => 0xB6,
            Self::LaxZpY(_) => 0xB7,
            Self::Clv => 0xB8,
            Self::LdaAbsY(_) => 0xB9,
            Self::Tsx => 0xBA,
            Self::Las(_) => 0xBB,
            Self::LdyAbsX(_) => 0xBC,
            Self::LdaAbsX(_) => 0xBD,
            Self::LdxAbsY(_) => 0xBE,
            Self::LaxAbsY(_) => 0xBF,
            Self::CpyImm(_) => 0xC0,
            Self::CmpIndX(_) => 0xC1,
            Self::NopImmC2(_) => 0xC2,
            Self::DcpIndX(_) => 0xC3,
            Self::CpyZp(_) => 0xC4,
            Self::CmpZp(_) => 0xC5,
            Self::DecZp(_) => 0xC6,
            Self::DcpZp(_) => 0xC7,
            Self::Iny => 0xC8,
            Self::CmpImm(_) => 0xC9,
            Self::Dex => 0xCA,
            Self::Axs(_) => 0xCB,
            Self::CpyAbs(_) => 0xCC,
            Self::CmpAbs(_) => 0xCD,
            Self::DecAbs(_) => 0xCE,
            Self::DcpAbs(_) => 0xCF,
            Self::Bne(_) => 0xD0,
            Self::CmpIndY(_) => 0xD1,
            Self::KilD2 => 0xD2,
            Self::DcpIndY(_) => 0xD3,
            Self::NopZpXD4(_) => 0xD4,
            Self::CmpZpX(_) => 0xD5,
            Self::DecZpX(_) => 0xD6,
            Self::DcpZpX(_) => 0xD7,
            Self::Cld => 0xD8,
            Self::CmpAbsY(_) => 0xD9,
            Self::NopDA => 0xDA,
            Self::DcpAbsY(_) => 0xDB,
            Self::NopAbsXDC(_) => 0xDC,
            Self::CmpAbsX(_) => 0xDD,
            Self::DecAbsX(_) => 0xDE,
            Self::DcpAbsX(_) => 0xDF,
            Self::CpxImm(_) => 0xE0,
            Self::SbcIndX(_) => 0xE1,
            Self::NopImmE2(_) => 0xE2,
            Self::IscIndX(_) => 0xE3,
            Self::CpxZp(_) => 0xE4,
            Self::SbcZp(_) => 0xE5,
            Self::IncZp(_) => 0xE6,
            Self::IscZp(_) => 0xE7,
            Self::Inx => 0xE8,
            Self::SbcImmE9(_) => 0xE9,
            Self::NopEA => 0xEA,
            Self::SbcImmEB(_) => 0xEB,
            Self::CpxAbs(_) => 0xEC,
            Self::SbcAbs(_) => 0xED,
            Self::IncAbs(_) => 0xEE,
            Self::IscAbs(_) => 0xEF,
            Self::Beq(_) => 0xF0,
            Self::SbcIndY(_) => 0xF1,
            Self::KilF2 => 0xF2,
            Self::IscIndY(_) => 0xF3,
            Self::NopZpXF4(_) => 0xF4,
            Self::SbcZpX(_) => 0xF5,
            Self::IncZpX(_) => 0xF6,
            Self::IscZpX(_) => 0xF7,
            Self::Sed => 0xF8,
            Self::SbcAbsY(_) => 0xF9,
            Self::NopFA => 0xFA,
            Self::IscAbsY(_) => 0xFB,
            Self::NopAbsXFC(_) => 0xFC,
            Self::SbcAbsX(_) => 0xFD,
            Self::IncAbsX(_) => 0xFE,
            Self::IscAbsX(_) => 0xFF,
        };

        Opcode::new(inner)
    }

    /// オペランドを返す。
    pub const fn operand(self) -> Operand {
        match self {
            Self::Brk => Operand::Imp,
            Self::OraIndX(zp) => Operand::IndX(zp),
            Self::Kil02 => Operand::Imp,
            Self::SloIndX(zp) => Operand::IndX(zp),
            Self::NopZp04(zp) => Operand::Zp(zp),
            Self::OraZp(zp) => Operand::Zp(zp),
            Self::AslZp(zp) => Operand::Zp(zp),
            Self::SloZp(zp) => Operand::Zp(zp),
            Self::Php => Operand::Imp,
            Self::OraImm(imm) => Operand::Imm(imm),
            Self::AslAcc => Operand::Acc,
            Self::Anc0B(imm) => Operand::Imm(imm),
            Self::NopAbs(abs) => Operand::Abs(abs),
            Self::OraAbs(abs) => Operand::Abs(abs),
            Self::AslAbs(abs) => Operand::Abs(abs),
            Self::SloAbs(abs) => Operand::Abs(abs),
            Self::Bpl(rel) => Operand::Rel(rel),
            Self::OraIndY(zp) => Operand::IndY(zp),
            Self::Kil12 => Operand::Imp,
            Self::SloIndY(zp) => Operand::IndY(zp),
            Self::NopZpX14(zp) => Operand::ZpX(zp),
            Self::OraZpX(zp) => Operand::ZpX(zp),
            Self::AslZpX(zp) => Operand::ZpX(zp),
            Self::SloZpX(zp) => Operand::ZpX(zp),
            Self::Clc => Operand::Imp,
            Self::OraAbsY(abs) => Operand::AbsY(abs),
            Self::Nop1A => Operand::Imp,
            Self::SloAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsX1C(abs) => Operand::AbsX(abs),
            Self::OraAbsX(abs) => Operand::AbsX(abs),
            Self::AslAbsX(abs) => Operand::AbsX(abs),
            Self::SloAbsX(abs) => Operand::AbsX(abs),
            Self::Jsr(abs) => Operand::Abs(abs),
            Self::AndIndX(zp) => Operand::IndX(zp),
            Self::Kil22 => Operand::Imp,
            Self::RlaIndX(zp) => Operand::IndX(zp),
            Self::BitZp(zp) => Operand::Zp(zp),
            Self::AndZp(zp) => Operand::Zp(zp),
            Self::RolZp(zp) => Operand::Zp(zp),
            Self::RlaZp(zp) => Operand::Zp(zp),
            Self::Plp => Operand::Imp,
            Self::AndImm(imm) => Operand::Imm(imm),
            Self::RolAcc => Operand::Acc,
            Self::Anc2B(imm) => Operand::Imm(imm),
            Self::BitAbs(abs) => Operand::Abs(abs),
            Self::AndAbs(abs) => Operand::Abs(abs),
            Self::RolAbs(abs) => Operand::Abs(abs),
            Self::RlaAbs(abs) => Operand::Abs(abs),
            Self::Bmi(rel) => Operand::Rel(rel),
            Self::AndIndY(zp) => Operand::IndY(zp),
            Self::Kil32 => Operand::Imp,
            Self::RlaIndY(zp) => Operand::IndY(zp),
            Self::NopZpX34(zp) => Operand::ZpX(zp),
            Self::AndZpX(zp) => Operand::ZpX(zp),
            Self::RolZpX(zp) => Operand::ZpX(zp),
            Self::RlaZpX(zp) => Operand::ZpX(zp),
            Self::Sec => Operand::Imp,
            Self::AndAbsY(abs) => Operand::AbsY(abs),
            Self::Nop3A => Operand::Imp,
            Self::RlaAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsX3C(abs) => Operand::AbsX(abs),
            Self::AndAbsX(abs) => Operand::AbsX(abs),
            Self::RolAbsX(abs) => Operand::AbsX(abs),
            Self::RlaAbsX(abs) => Operand::AbsX(abs),
            Self::Rti => Operand::Imp,
            Self::EorIndX(zp) => Operand::IndX(zp),
            Self::Kil42 => Operand::Imp,
            Self::SreIndX(zp) => Operand::IndX(zp),
            Self::NopZp44(zp) => Operand::Zp(zp),
            Self::EorZp(zp) => Operand::Zp(zp),
            Self::LsrZp(zp) => Operand::Zp(zp),
            Self::SreZp(zp) => Operand::Zp(zp),
            Self::Pha => Operand::Imp,
            Self::EorImm(imm) => Operand::Imm(imm),
            Self::LsrAcc => Operand::Acc,
            Self::Alr(imm) => Operand::Imm(imm),
            Self::JmpAbs(abs) => Operand::Abs(abs),
            Self::EorAbs(abs) => Operand::Abs(abs),
            Self::LsrAbs(abs) => Operand::Abs(abs),
            Self::SreAbs(abs) => Operand::Abs(abs),
            Self::Bvc(rel) => Operand::Rel(rel),
            Self::EorIndY(zp) => Operand::IndY(zp),
            Self::Kil52 => Operand::Imp,
            Self::SreIndY(zp) => Operand::IndY(zp),
            Self::NopZpX54(zp) => Operand::ZpX(zp),
            Self::EorZpX(zp) => Operand::ZpX(zp),
            Self::LsrZpX(zp) => Operand::ZpX(zp),
            Self::SreZpX(zp) => Operand::ZpX(zp),
            Self::Cli => Operand::Imp,
            Self::EorAbsY(abs) => Operand::AbsY(abs),
            Self::Nop5A => Operand::Imp,
            Self::SreAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsX5C(abs) => Operand::AbsX(abs),
            Self::EorAbsX(abs) => Operand::AbsX(abs),
            Self::LsrAbsX(abs) => Operand::AbsX(abs),
            Self::SreAbsX(abs) => Operand::AbsX(abs),
            Self::Rts => Operand::Imp,
            Self::AdcIndX(zp) => Operand::IndX(zp),
            Self::Kil62 => Operand::Imp,
            Self::RraIndX(zp) => Operand::IndX(zp),
            Self::NopZp64(zp) => Operand::Zp(zp),
            Self::AdcZp(zp) => Operand::Zp(zp),
            Self::RorZp(zp) => Operand::Zp(zp),
            Self::RraZp(zp) => Operand::Zp(zp),
            Self::Pla => Operand::Imp,
            Self::AdcImm(imm) => Operand::Imm(imm),
            Self::RorAcc => Operand::Acc,
            Self::Arr(imm) => Operand::Imm(imm),
            Self::JmpInd(abs) => Operand::Ind(abs),
            Self::AdcAbs(abs) => Operand::Abs(abs),
            Self::RorAbs(abs) => Operand::Abs(abs),
            Self::RraAbs(abs) => Operand::Abs(abs),
            Self::Bvs(rel) => Operand::Rel(rel),
            Self::AdcIndY(zp) => Operand::IndY(zp),
            Self::Kil72 => Operand::Imp,
            Self::RraIndY(zp) => Operand::IndY(zp),
            Self::NopZpX74(zp) => Operand::ZpX(zp),
            Self::AdcZpX(zp) => Operand::ZpX(zp),
            Self::RorZpX(zp) => Operand::ZpX(zp),
            Self::RraZpX(zp) => Operand::ZpX(zp),
            Self::Sei => Operand::Imp,
            Self::AdcAbsY(abs) => Operand::AbsY(abs),
            Self::Nop7A => Operand::Imp,
            Self::RraAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsX7C(abs) => Operand::AbsX(abs),
            Self::AdcAbsX(abs) => Operand::AbsX(abs),
            Self::RorAbsX(abs) => Operand::AbsX(abs),
            Self::RraAbsX(abs) => Operand::AbsX(abs),
            Self::NopImm80(imm) => Operand::Imm(imm),
            Self::StaIndX(zp) => Operand::IndX(zp),
            Self::NopImm82(imm) => Operand::Imm(imm),
            Self::SaxIndX(zp) => Operand::IndX(zp),
            Self::StyZp(zp) => Operand::Zp(zp),
            Self::StaZp(zp) => Operand::Zp(zp),
            Self::StxZp(zp) => Operand::Zp(zp),
            Self::SaxZp(zp) => Operand::Zp(zp),
            Self::Dey => Operand::Imp,
            Self::NopImm89(imm) => Operand::Imm(imm),
            Self::Txa => Operand::Imp,
            Self::Xaa(imm) => Operand::Imm(imm),
            Self::StyAbs(abs) => Operand::Abs(abs),
            Self::StaAbs(abs) => Operand::Abs(abs),
            Self::StxAbs(abs) => Operand::Abs(abs),
            Self::SaxAbs(abs) => Operand::Abs(abs),
            Self::Bcc(rel) => Operand::Rel(rel),
            Self::StaIndY(zp) => Operand::IndY(zp),
            Self::Kil92 => Operand::Imp,
            Self::AhxIndY(zp) => Operand::IndY(zp),
            Self::StyZpX(zp) => Operand::ZpX(zp),
            Self::StaZpX(zp) => Operand::ZpX(zp),
            Self::StxZpY(zp) => Operand::ZpY(zp),
            Self::SaxZpY(zp) => Operand::ZpY(zp),
            Self::Tya => Operand::Imp,
            Self::StaAbsY(abs) => Operand::AbsY(abs),
            Self::Txs => Operand::Imp,
            Self::Tas(abs) => Operand::AbsY(abs),
            Self::Shy(abs) => Operand::AbsX(abs),
            Self::StaAbsX(abs) => Operand::AbsX(abs),
            Self::Shx(abs) => Operand::AbsY(abs),
            Self::AhxAbsY(abs) => Operand::AbsY(abs),
            Self::LdyImm(imm) => Operand::Imm(imm),
            Self::LdaIndX(zp) => Operand::IndX(zp),
            Self::LdxImm(imm) => Operand::Imm(imm),
            Self::LaxIndX(zp) => Operand::IndX(zp),
            Self::LdyZp(zp) => Operand::Zp(zp),
            Self::LdaZp(zp) => Operand::Zp(zp),
            Self::LdxZp(zp) => Operand::Zp(zp),
            Self::LaxZp(zp) => Operand::Zp(zp),
            Self::Tay => Operand::Imp,
            Self::LdaImm(imm) => Operand::Imm(imm),
            Self::Tax => Operand::Imp,
            Self::LaxImm(imm) => Operand::Imm(imm),
            Self::LdyAbs(abs) => Operand::Abs(abs),
            Self::LdaAbs(abs) => Operand::Abs(abs),
            Self::LdxAbs(abs) => Operand::Abs(abs),
            Self::LaxAbs(abs) => Operand::Abs(abs),
            Self::Bcs(rel) => Operand::Rel(rel),
            Self::LdaIndY(zp) => Operand::IndY(zp),
            Self::KilB2 => Operand::Imp,
            Self::LaxIndY(zp) => Operand::IndY(zp),
            Self::LdyZpX(zp) => Operand::ZpX(zp),
            Self::LdaZpX(zp) => Operand::ZpX(zp),
            Self::LdxZpY(zp) => Operand::ZpY(zp),
            Self::LaxZpY(zp) => Operand::ZpY(zp),
            Self::Clv => Operand::Imp,
            Self::LdaAbsY(abs) => Operand::AbsY(abs),
            Self::Tsx => Operand::Imp,
            Self::Las(abs) => Operand::AbsY(abs),
            Self::LdyAbsX(abs) => Operand::AbsX(abs),
            Self::LdaAbsX(abs) => Operand::AbsX(abs),
            Self::LdxAbsY(abs) => Operand::AbsY(abs),
            Self::LaxAbsY(abs) => Operand::AbsY(abs),
            Self::CpyImm(imm) => Operand::Imm(imm),
            Self::CmpIndX(zp) => Operand::IndX(zp),
            Self::NopImmC2(imm) => Operand::Imm(imm),
            Self::DcpIndX(zp) => Operand::IndX(zp),
            Self::CpyZp(zp) => Operand::Zp(zp),
            Self::CmpZp(zp) => Operand::Zp(zp),
            Self::DecZp(zp) => Operand::Zp(zp),
            Self::DcpZp(zp) => Operand::Zp(zp),
            Self::Iny => Operand::Imp,
            Self::CmpImm(imm) => Operand::Imm(imm),
            Self::Dex => Operand::Imp,
            Self::Axs(imm) => Operand::Imm(imm),
            Self::CpyAbs(abs) => Operand::Abs(abs),
            Self::CmpAbs(abs) => Operand::Abs(abs),
            Self::DecAbs(abs) => Operand::Abs(abs),
            Self::DcpAbs(abs) => Operand::Abs(abs),
            Self::Bne(rel) => Operand::Rel(rel),
            Self::CmpIndY(zp) => Operand::IndY(zp),
            Self::KilD2 => Operand::Imp,
            Self::DcpIndY(zp) => Operand::IndY(zp),
            Self::NopZpXD4(zp) => Operand::ZpX(zp),
            Self::CmpZpX(zp) => Operand::ZpX(zp),
            Self::DecZpX(zp) => Operand::ZpX(zp),
            Self::DcpZpX(zp) => Operand::ZpX(zp),
            Self::Cld => Operand::Imp,
            Self::CmpAbsY(abs) => Operand::AbsY(abs),
            Self::NopDA => Operand::Imp,
            Self::DcpAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsXDC(abs) => Operand::AbsX(abs),
            Self::CmpAbsX(abs) => Operand::AbsX(abs),
            Self::DecAbsX(abs) => Operand::AbsX(abs),
            Self::DcpAbsX(abs) => Operand::AbsX(abs),
            Self::CpxImm(imm) => Operand::Imm(imm),
            Self::SbcIndX(zp) => Operand::IndX(zp),
            Self::NopImmE2(imm) => Operand::Imm(imm),
            Self::IscIndX(zp) => Operand::IndX(zp),
            Self::CpxZp(zp) => Operand::Zp(zp),
            Self::SbcZp(zp) => Operand::Zp(zp),
            Self::IncZp(zp) => Operand::Zp(zp),
            Self::IscZp(zp) => Operand::Zp(zp),
            Self::Inx => Operand::Imp,
            Self::SbcImmE9(imm) => Operand::Imm(imm),
            Self::NopEA => Operand::Imp,
            Self::SbcImmEB(imm) => Operand::Imm(imm),
            Self::CpxAbs(abs) => Operand::Abs(abs),
            Self::SbcAbs(abs) => Operand::Abs(abs),
            Self::IncAbs(abs) => Operand::Abs(abs),
            Self::IscAbs(abs) => Operand::Abs(abs),
            Self::Beq(rel) => Operand::Rel(rel),
            Self::SbcIndY(zp) => Operand::IndY(zp),
            Self::KilF2 => Operand::Imp,
            Self::IscIndY(zp) => Operand::IndY(zp),
            Self::NopZpXF4(zp) => Operand::ZpX(zp),
            Self::SbcZpX(zp) => Operand::ZpX(zp),
            Self::IncZpX(zp) => Operand::ZpX(zp),
            Self::IscZpX(zp) => Operand::ZpX(zp),
            Self::Sed => Operand::Imp,
            Self::SbcAbsY(abs) => Operand::AbsY(abs),
            Self::NopFA => Operand::Imp,
            Self::IscAbsY(abs) => Operand::AbsY(abs),
            Self::NopAbsXFC(abs) => Operand::AbsX(abs),
            Self::SbcAbsX(abs) => Operand::AbsX(abs),
            Self::IncAbsX(abs) => Operand::AbsX(abs),
            Self::IscAbsX(abs) => Operand::AbsX(abs),
        }
    }

    /// 命令全体のバイト数を返す。
    pub const fn len(self) -> NonZeroUsize {
        self.opcode().op_len()
    }

    /// 公式命令かどうかを返す。
    pub const fn is_official(self) -> bool {
        self.opcode().is_official()
    }

    /// オペランドの実効アドレスに対する読み取りを行うかどうかを返す。
    /// (命令フェッチや間接アドレッシングにおけるポインタ自体の読み取りは含まない)
    ///
    /// 実効アドレスのない命令 (implied, accumulator, immediate, relative) の場合、`false` を返す。
    pub const fn is_read(self) -> bool {
        self.opcode().is_read()
    }

    /// オペランドの実効アドレスに対する書き込みを行うかどうかを返す。
    ///
    /// 実効アドレスのない命令 (implied, accumulator, immediate, relative) の場合、`false` を返す。
    pub const fn is_write(self) -> bool {
        self.opcode().is_write()
    }

    /// 分岐命令かどうかを返す。
    pub const fn is_branch(self) -> bool {
        self.opcode().is_branch()
    }

    /// kil 命令かどうかを返す。
    pub const fn is_kil(self) -> bool {
        self.opcode().is_kil()
    }

    /// 制御フロー命令かどうかを返す。
    ///
    /// 以下の命令が該当する:
    ///
    /// * brk
    /// * jsr
    /// * rti
    /// * jmp abs
    /// * rts
    /// * jmp ind
    /// * 全ての分岐命令
    /// * 全ての kil 命令
    pub const fn is_flow(self) -> bool {
        self.opcode().is_flow()
    }

    /// 即値オペランドをとるビット演算命令かどうかを返す。
    pub const fn is_bitop_imm(self) -> bool {
        matches!(
            self,
            Self::OraImm(_)
                | Self::Anc0B(_)
                | Self::AndImm(_)
                | Self::Anc2B(_)
                | Self::EorImm(_)
                | Self::Alr(_)
                | Self::Arr(_)
                | Self::Xaa(_)
        )
    }

    /// 分岐命令ならばその分岐オフセットを返す。
    pub const fn match_branch(self) -> Option<i8> {
        match self {
            Self::Bpl(rel)
            | Self::Bmi(rel)
            | Self::Bvc(rel)
            | Self::Bvs(rel)
            | Self::Bcc(rel)
            | Self::Bcs(rel)
            | Self::Bne(rel)
            | Self::Beq(rel) => Some(rel),
            _ => None,
        }
    }

    /// 命令を実行した後のプログラムカウンタの動きを返す。
    pub const fn succ(self) -> OpSucc {
        if let Some(rel) = self.match_branch() {
            OpSucc::Branch(rel)
        } else {
            match self {
                Self::Brk => OpSucc::Brk,
                Self::Jsr(abs) => OpSucc::Jsr(abs),
                Self::Rti => OpSucc::Rti,
                Self::Rts => OpSucc::Rts,
                Self::JmpAbs(abs) => OpSucc::JmpAbs(abs),
                Self::JmpInd(abs) => OpSucc::JmpInd(abs),
                _ if self.is_kil() => OpSucc::Kil,
                _ => OpSucc::Normal(self.len()),
            }
        }
    }

    /// 命令を機械語のバイト列に変換する。
    pub fn to_bytes(self) -> ArrayVec<u8, 3> {
        std::iter::once(self.opcode().get())
            .chain(self.operand().to_bytes())
            .collect()
    }

    /// 機械語のバイト列を命令に変換する。
    /// バイト数が命令長と一致しない場合、panic する。
    pub fn fetch(buf: &[u8]) -> Self {
        assert!(!buf.is_empty(), "op buffer is empty");

        let (&opcode, buf) = buf.split_first().unwrap();
        let opcode = Opcode::new(opcode);
        assert_eq!(
            buf.len(),
            opcode.operand_len(),
            "op buffer length is invalid (opcode={:#X})",
            opcode.get(),
        );

        let fetch_imm = || buf[0];
        let fetch_zp = || ZpAddress::new(buf[0]);
        let fetch_abs = || {
            let buf: [u8; 2] = buf[..2].try_into().unwrap();
            Address::from_le_bytes(buf)
        };
        let fetch_rel = || buf[0] as i8;

        match opcode.get() {
            0x00 => Self::Brk,
            0x01 => Self::OraIndX(fetch_zp()),
            0x02 => Self::Kil02,
            0x03 => Self::SloIndX(fetch_zp()),
            0x04 => Self::NopZp04(fetch_zp()),
            0x05 => Self::OraZp(fetch_zp()),
            0x06 => Self::AslZp(fetch_zp()),
            0x07 => Self::SloZp(fetch_zp()),
            0x08 => Self::Php,
            0x09 => Self::OraImm(fetch_imm()),
            0x0A => Self::AslAcc,
            0x0B => Self::Anc0B(fetch_imm()),
            0x0C => Self::NopAbs(fetch_abs()),
            0x0D => Self::OraAbs(fetch_abs()),
            0x0E => Self::AslAbs(fetch_abs()),
            0x0F => Self::SloAbs(fetch_abs()),
            0x10 => Self::Bpl(fetch_rel()),
            0x11 => Self::OraIndY(fetch_zp()),
            0x12 => Self::Kil12,
            0x13 => Self::SloIndY(fetch_zp()),
            0x14 => Self::NopZpX14(fetch_zp()),
            0x15 => Self::OraZpX(fetch_zp()),
            0x16 => Self::AslZpX(fetch_zp()),
            0x17 => Self::SloZpX(fetch_zp()),
            0x18 => Self::Clc,
            0x19 => Self::OraAbsY(fetch_abs()),
            0x1A => Self::Nop1A,
            0x1B => Self::SloAbsY(fetch_abs()),
            0x1C => Self::NopAbsX1C(fetch_abs()),
            0x1D => Self::OraAbsX(fetch_abs()),
            0x1E => Self::AslAbsX(fetch_abs()),
            0x1F => Self::SloAbsX(fetch_abs()),
            0x20 => Self::Jsr(fetch_abs()),
            0x21 => Self::AndIndX(fetch_zp()),
            0x22 => Self::Kil22,
            0x23 => Self::RlaIndX(fetch_zp()),
            0x24 => Self::BitZp(fetch_zp()),
            0x25 => Self::AndZp(fetch_zp()),
            0x26 => Self::RolZp(fetch_zp()),
            0x27 => Self::RlaZp(fetch_zp()),
            0x28 => Self::Plp,
            0x29 => Self::AndImm(fetch_imm()),
            0x2A => Self::RolAcc,
            0x2B => Self::Anc2B(fetch_imm()),
            0x2C => Self::BitAbs(fetch_abs()),
            0x2D => Self::AndAbs(fetch_abs()),
            0x2E => Self::RolAbs(fetch_abs()),
            0x2F => Self::RlaAbs(fetch_abs()),
            0x30 => Self::Bmi(fetch_rel()),
            0x31 => Self::AndIndY(fetch_zp()),
            0x32 => Self::Kil32,
            0x33 => Self::RlaIndY(fetch_zp()),
            0x34 => Self::NopZpX34(fetch_zp()),
            0x35 => Self::AndZpX(fetch_zp()),
            0x36 => Self::RolZpX(fetch_zp()),
            0x37 => Self::RlaZpX(fetch_zp()),
            0x38 => Self::Sec,
            0x39 => Self::AndAbsY(fetch_abs()),
            0x3A => Self::Nop3A,
            0x3B => Self::RlaAbsY(fetch_abs()),
            0x3C => Self::NopAbsX3C(fetch_abs()),
            0x3D => Self::AndAbsX(fetch_abs()),
            0x3E => Self::RolAbsX(fetch_abs()),
            0x3F => Self::RlaAbsX(fetch_abs()),
            0x40 => Self::Rti,
            0x41 => Self::EorIndX(fetch_zp()),
            0x42 => Self::Kil42,
            0x43 => Self::SreIndX(fetch_zp()),
            0x44 => Self::NopZp44(fetch_zp()),
            0x45 => Self::EorZp(fetch_zp()),
            0x46 => Self::LsrZp(fetch_zp()),
            0x47 => Self::SreZp(fetch_zp()),
            0x48 => Self::Pha,
            0x49 => Self::EorImm(fetch_imm()),
            0x4A => Self::LsrAcc,
            0x4B => Self::Alr(fetch_imm()),
            0x4C => Self::JmpAbs(fetch_abs()),
            0x4D => Self::EorAbs(fetch_abs()),
            0x4E => Self::LsrAbs(fetch_abs()),
            0x4F => Self::SreAbs(fetch_abs()),
            0x50 => Self::Bvc(fetch_rel()),
            0x51 => Self::EorIndY(fetch_zp()),
            0x52 => Self::Kil52,
            0x53 => Self::SreIndY(fetch_zp()),
            0x54 => Self::NopZpX54(fetch_zp()),
            0x55 => Self::EorZpX(fetch_zp()),
            0x56 => Self::LsrZpX(fetch_zp()),
            0x57 => Self::SreZpX(fetch_zp()),
            0x58 => Self::Cli,
            0x59 => Self::EorAbsY(fetch_abs()),
            0x5A => Self::Nop5A,
            0x5B => Self::SreAbsY(fetch_abs()),
            0x5C => Self::NopAbsX5C(fetch_abs()),
            0x5D => Self::EorAbsX(fetch_abs()),
            0x5E => Self::LsrAbsX(fetch_abs()),
            0x5F => Self::SreAbsX(fetch_abs()),
            0x60 => Self::Rts,
            0x61 => Self::AdcIndX(fetch_zp()),
            0x62 => Self::Kil62,
            0x63 => Self::RraIndX(fetch_zp()),
            0x64 => Self::NopZp64(fetch_zp()),
            0x65 => Self::AdcZp(fetch_zp()),
            0x66 => Self::RorZp(fetch_zp()),
            0x67 => Self::RraZp(fetch_zp()),
            0x68 => Self::Pla,
            0x69 => Self::AdcImm(fetch_imm()),
            0x6A => Self::RorAcc,
            0x6B => Self::Arr(fetch_imm()),
            0x6C => Self::JmpInd(fetch_abs()),
            0x6D => Self::AdcAbs(fetch_abs()),
            0x6E => Self::RorAbs(fetch_abs()),
            0x6F => Self::RraAbs(fetch_abs()),
            0x70 => Self::Bvs(fetch_rel()),
            0x71 => Self::AdcIndY(fetch_zp()),
            0x72 => Self::Kil72,
            0x73 => Self::RraIndY(fetch_zp()),
            0x74 => Self::NopZpX74(fetch_zp()),
            0x75 => Self::AdcZpX(fetch_zp()),
            0x76 => Self::RorZpX(fetch_zp()),
            0x77 => Self::RraZpX(fetch_zp()),
            0x78 => Self::Sei,
            0x79 => Self::AdcAbsY(fetch_abs()),
            0x7A => Self::Nop7A,
            0x7B => Self::RraAbsY(fetch_abs()),
            0x7C => Self::NopAbsX7C(fetch_abs()),
            0x7D => Self::AdcAbsX(fetch_abs()),
            0x7E => Self::RorAbsX(fetch_abs()),
            0x7F => Self::RraAbsX(fetch_abs()),
            0x80 => Self::NopImm80(fetch_imm()),
            0x81 => Self::StaIndX(fetch_zp()),
            0x82 => Self::NopImm82(fetch_imm()),
            0x83 => Self::SaxIndX(fetch_zp()),
            0x84 => Self::StyZp(fetch_zp()),
            0x85 => Self::StaZp(fetch_zp()),
            0x86 => Self::StxZp(fetch_zp()),
            0x87 => Self::SaxZp(fetch_zp()),
            0x88 => Self::Dey,
            0x89 => Self::NopImm89(fetch_imm()),
            0x8A => Self::Txa,
            0x8B => Self::Xaa(fetch_imm()),
            0x8C => Self::StyAbs(fetch_abs()),
            0x8D => Self::StaAbs(fetch_abs()),
            0x8E => Self::StxAbs(fetch_abs()),
            0x8F => Self::SaxAbs(fetch_abs()),
            0x90 => Self::Bcc(fetch_rel()),
            0x91 => Self::StaIndY(fetch_zp()),
            0x92 => Self::Kil92,
            0x93 => Self::AhxIndY(fetch_zp()),
            0x94 => Self::StyZpX(fetch_zp()),
            0x95 => Self::StaZpX(fetch_zp()),
            0x96 => Self::StxZpY(fetch_zp()),
            0x97 => Self::SaxZpY(fetch_zp()),
            0x98 => Self::Tya,
            0x99 => Self::StaAbsY(fetch_abs()),
            0x9A => Self::Txs,
            0x9B => Self::Tas(fetch_abs()),
            0x9C => Self::Shy(fetch_abs()),
            0x9D => Self::StaAbsX(fetch_abs()),
            0x9E => Self::Shx(fetch_abs()),
            0x9F => Self::AhxAbsY(fetch_abs()),
            0xA0 => Self::LdyImm(fetch_imm()),
            0xA1 => Self::LdaIndX(fetch_zp()),
            0xA2 => Self::LdxImm(fetch_imm()),
            0xA3 => Self::LaxIndX(fetch_zp()),
            0xA4 => Self::LdyZp(fetch_zp()),
            0xA5 => Self::LdaZp(fetch_zp()),
            0xA6 => Self::LdxZp(fetch_zp()),
            0xA7 => Self::LaxZp(fetch_zp()),
            0xA8 => Self::Tay,
            0xA9 => Self::LdaImm(fetch_imm()),
            0xAA => Self::Tax,
            0xAB => Self::LaxImm(fetch_imm()),
            0xAC => Self::LdyAbs(fetch_abs()),
            0xAD => Self::LdaAbs(fetch_abs()),
            0xAE => Self::LdxAbs(fetch_abs()),
            0xAF => Self::LaxAbs(fetch_abs()),
            0xB0 => Self::Bcs(fetch_rel()),
            0xB1 => Self::LdaIndY(fetch_zp()),
            0xB2 => Self::KilB2,
            0xB3 => Self::LaxIndY(fetch_zp()),
            0xB4 => Self::LdyZpX(fetch_zp()),
            0xB5 => Self::LdaZpX(fetch_zp()),
            0xB6 => Self::LdxZpY(fetch_zp()),
            0xB7 => Self::LaxZpY(fetch_zp()),
            0xB8 => Self::Clv,
            0xB9 => Self::LdaAbsY(fetch_abs()),
            0xBA => Self::Tsx,
            0xBB => Self::Las(fetch_abs()),
            0xBC => Self::LdyAbsX(fetch_abs()),
            0xBD => Self::LdaAbsX(fetch_abs()),
            0xBE => Self::LdxAbsY(fetch_abs()),
            0xBF => Self::LaxAbsY(fetch_abs()),
            0xC0 => Self::CpyImm(fetch_imm()),
            0xC1 => Self::CmpIndX(fetch_zp()),
            0xC2 => Self::NopImmC2(fetch_imm()),
            0xC3 => Self::DcpIndX(fetch_zp()),
            0xC4 => Self::CpyZp(fetch_zp()),
            0xC5 => Self::CmpZp(fetch_zp()),
            0xC6 => Self::DecZp(fetch_zp()),
            0xC7 => Self::DcpZp(fetch_zp()),
            0xC8 => Self::Iny,
            0xC9 => Self::CmpImm(fetch_imm()),
            0xCA => Self::Dex,
            0xCB => Self::Axs(fetch_imm()),
            0xCC => Self::CpyAbs(fetch_abs()),
            0xCD => Self::CmpAbs(fetch_abs()),
            0xCE => Self::DecAbs(fetch_abs()),
            0xCF => Self::DcpAbs(fetch_abs()),
            0xD0 => Self::Bne(fetch_rel()),
            0xD1 => Self::CmpIndY(fetch_zp()),
            0xD2 => Self::KilD2,
            0xD3 => Self::DcpIndY(fetch_zp()),
            0xD4 => Self::NopZpXD4(fetch_zp()),
            0xD5 => Self::CmpZpX(fetch_zp()),
            0xD6 => Self::DecZpX(fetch_zp()),
            0xD7 => Self::DcpZpX(fetch_zp()),
            0xD8 => Self::Cld,
            0xD9 => Self::CmpAbsY(fetch_abs()),
            0xDA => Self::NopDA,
            0xDB => Self::DcpAbsY(fetch_abs()),
            0xDC => Self::NopAbsXDC(fetch_abs()),
            0xDD => Self::CmpAbsX(fetch_abs()),
            0xDE => Self::DecAbsX(fetch_abs()),
            0xDF => Self::DcpAbsX(fetch_abs()),
            0xE0 => Self::CpxImm(fetch_imm()),
            0xE1 => Self::SbcIndX(fetch_zp()),
            0xE2 => Self::NopImmE2(fetch_imm()),
            0xE3 => Self::IscIndX(fetch_zp()),
            0xE4 => Self::CpxZp(fetch_zp()),
            0xE5 => Self::SbcZp(fetch_zp()),
            0xE6 => Self::IncZp(fetch_zp()),
            0xE7 => Self::IscZp(fetch_zp()),
            0xE8 => Self::Inx,
            0xE9 => Self::SbcImmE9(fetch_imm()),
            0xEA => Self::NopEA,
            0xEB => Self::SbcImmEB(fetch_imm()),
            0xEC => Self::CpxAbs(fetch_abs()),
            0xED => Self::SbcAbs(fetch_abs()),
            0xEE => Self::IncAbs(fetch_abs()),
            0xEF => Self::IscAbs(fetch_abs()),
            0xF0 => Self::Beq(fetch_rel()),
            0xF1 => Self::SbcIndY(fetch_zp()),
            0xF2 => Self::KilF2,
            0xF3 => Self::IscIndY(fetch_zp()),
            0xF4 => Self::NopZpXF4(fetch_zp()),
            0xF5 => Self::SbcZpX(fetch_zp()),
            0xF6 => Self::IncZpX(fetch_zp()),
            0xF7 => Self::IscZpX(fetch_zp()),
            0xF8 => Self::Sed,
            0xF9 => Self::SbcAbsY(fetch_abs()),
            0xFA => Self::NopFA,
            0xFB => Self::IscAbsY(fetch_abs()),
            0xFC => Self::NopAbsXFC(fetch_abs()),
            0xFD => Self::SbcAbsX(fetch_abs()),
            0xFE => Self::IncAbsX(fetch_abs()),
            0xFF => Self::IscAbsX(fetch_abs()),
        }
    }
}

/// CPU 命令のオペコード。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Opcode(u8);

impl Opcode {
    pub const BRK: Self = Self(0x00);
    pub const BPL: Self = Self(0x10);
    pub const JSR: Self = Self(0x20);
    pub const BMI: Self = Self(0x30);
    pub const RTI: Self = Self(0x40);
    pub const JMP_ABS: Self = Self(0x4C);
    pub const BVC: Self = Self(0x50);
    pub const RTS: Self = Self(0x60);
    pub const JMP_IND: Self = Self(0x6C);
    pub const BVS: Self = Self(0x70);
    pub const BCC: Self = Self(0x90);
    pub const BCS: Self = Self(0xB0);
    pub const CLV: Self = Self(0xB8);
    pub const BNE: Self = Self(0xD0);
    pub const BEQ: Self = Self(0xF0);
    pub const SED: Self = Self(0xF8);

    pub const fn new(inner: u8) -> Self {
        Self(inner)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    /// 命令全体のバイト数を返す。
    pub const fn op_len(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1 + self.operand_len()) }
    }

    /// オペランドのバイト数を返す。
    pub const fn operand_len(self) -> usize {
        self.addressing().operand_len()
    }

    /// 公式命令かどうかを返す。
    pub const fn is_official(self) -> bool {
        #[rustfmt::skip]
        const TABLE: [u8; 0x100] = [
            /* 0x00 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0,
            /* 0x10 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            /* 0x20 */ 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0x30 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            /* 0x40 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0x50 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            /* 0x60 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0x70 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            /* 0x80 */ 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0,
            /* 0x90 */ 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0,
            /* 0xA0 */ 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0xB0 */ 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0xC0 */ 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0xD0 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            /* 0xE0 */ 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
            /* 0xF0 */ 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0,
        ];

        TABLE[self.0 as usize] != 0
    }

    /// オペランドの実効アドレスに対する読み取りを行うかどうかを返す。
    /// (命令フェッチや間接アドレッシングにおけるポインタ自体の読み取りは含まない)
    ///
    /// 実効アドレスのない命令 (implied, accumulator, immediate, relative) の場合、`false` を返す。
    pub const fn is_read(self) -> bool {
        #[rustfmt::skip]
        const TABLE: [u8; 0x100] = [
            /* 0x00 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0x10 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0x20 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0x30 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0x40 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1,
            /* 0x50 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0x60 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1,
            /* 0x70 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0x80 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            /* 0x90 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            /* 0xA0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0xB0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0xC0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0xD0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0xE0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0xF0 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
        ];

        TABLE[self.0 as usize] != 0
    }

    /// オペランドの実効アドレスに対する書き込みを行うかどうかを返す。
    ///
    /// 実効アドレスのない命令 (implied, accumulator, immediate, relative) の場合、`false` を返す。
    pub const fn is_write(self) -> bool {
        #[rustfmt::skip]
        const TABLE: [u8; 0x100] = [
            /* 0x00 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0x10 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
            /* 0x20 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0x30 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
            /* 0x40 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0x50 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
            /* 0x60 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0x70 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
            /* 0x80 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            /* 0x90 */ 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
            /* 0xA0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            /* 0xB0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            /* 0xC0 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0xD0 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
            /* 0xE0 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            /* 0xF0 */ 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1,
        ];

        TABLE[self.0 as usize] != 0
    }

    /// 分岐命令かどうかを返す。
    pub const fn is_branch(self) -> bool {
        matches!(
            self,
            Self::BPL
                | Self::BMI
                | Self::BVC
                | Self::BVS
                | Self::BCC
                | Self::BCS
                | Self::BNE
                | Self::BEQ
        )
    }

    /// kil 命令かどうかを返す。
    pub const fn is_kil(self) -> bool {
        matches!(
            self.0,
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2
        )
    }

    /// 制御フロー命令かどうかを返す。
    ///
    /// 以下の命令が該当する:
    ///
    /// * brk
    /// * jsr
    /// * rti
    /// * jmp abs
    /// * rts
    /// * jmp ind
    /// * 全ての分岐命令
    /// * 全ての kil 命令
    pub const fn is_flow(self) -> bool {
        matches!(
            self,
            Self::BRK | Self::JSR | Self::RTI | Self::JMP_ABS | Self::RTS | Self::JMP_IND
        ) || self.is_branch()
            || self.is_kil()
    }

    /// ニーモニック文字列を返す。
    pub const fn mnemonic(self) -> &'static str {
        #[rustfmt::skip]
        const TABLE: [&str; 0x100] = [
            /* 0x00 */ "brk", "ora", "kil", "slo", "nop", "ora", "asl", "slo", "php", "ora", "asl", "anc", "nop", "ora", "asl", "slo",
            /* 0x10 */ "bpl", "ora", "kil", "slo", "nop", "ora", "asl", "slo", "clc", "ora", "nop", "slo", "nop", "ora", "asl", "slo",
            /* 0x20 */ "jsr", "and", "kil", "rla", "bit", "and", "rol", "rla", "plp", "and", "rol", "anc", "bit", "and", "rol", "rla",
            /* 0x30 */ "bmi", "and", "kil", "rla", "nop", "and", "rol", "rla", "sec", "and", "nop", "rla", "nop", "and", "rol", "rla",
            /* 0x40 */ "rti", "eor", "kil", "sre", "nop", "eor", "lsr", "sre", "pha", "eor", "lsr", "alr", "jmp", "eor", "lsr", "sre",
            /* 0x50 */ "bvc", "eor", "kil", "sre", "nop", "eor", "lsr", "sre", "cli", "eor", "nop", "sre", "nop", "eor", "lsr", "sre",
            /* 0x60 */ "rts", "adc", "kil", "rra", "nop", "adc", "ror", "rra", "pla", "adc", "ror", "arr", "jmp", "adc", "ror", "rra",
            /* 0x70 */ "bvs", "adc", "kil", "rra", "nop", "adc", "ror", "rra", "sei", "adc", "nop", "rra", "nop", "adc", "ror", "rra",
            /* 0x80 */ "nop", "sta", "nop", "sax", "sty", "sta", "stx", "sax", "dey", "nop", "txa", "xaa", "sty", "sta", "stx", "sax",
            /* 0x90 */ "bcc", "sta", "kil", "ahx", "sty", "sta", "stx", "sax", "tya", "sta", "txs", "tas", "shy", "sta", "shx", "ahx",
            /* 0xA0 */ "ldy", "lda", "ldx", "lax", "ldy", "lda", "ldx", "lax", "tay", "lda", "tax", "lax", "ldy", "lda", "ldx", "lax",
            /* 0xB0 */ "bcs", "lda", "kil", "lax", "ldy", "lda", "ldx", "lax", "clv", "lda", "tsx", "las", "ldy", "lda", "ldx", "lax",
            /* 0xC0 */ "cpy", "cmp", "nop", "dcp", "cpy", "cmp", "dec", "dcp", "iny", "cmp", "dex", "axs", "cpy", "cmp", "dec", "dcp",
            /* 0xD0 */ "bne", "cmp", "kil", "dcp", "nop", "cmp", "dec", "dcp", "cld", "cmp", "nop", "dcp", "nop", "cmp", "dec", "dcp",
            /* 0xE0 */ "cpx", "sbc", "nop", "isc", "cpx", "sbc", "inc", "isc", "inx", "sbc", "nop", "sbc", "cpx", "sbc", "inc", "isc",
            /* 0xF0 */ "beq", "sbc", "kil", "isc", "nop", "sbc", "inc", "isc", "sed", "sbc", "nop", "isc", "nop", "sbc", "inc", "isc",
        ];

        TABLE[self.0 as usize]
    }

    /// アドレッシングモードを返す。
    const fn addressing(self) -> Addressing {
        use Addressing::*;

        #[rustfmt::skip]
        const TABLE: [Addressing; 0x100] = [
            /* 0x00 */ Imp, IndX, Imp, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Acc, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0x10 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
            /* 0x20 */ Abs, IndX, Imp, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Acc, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0x30 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
            /* 0x40 */ Imp, IndX, Imp, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Acc, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0x50 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
            /* 0x60 */ Imp, IndX, Imp, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Acc, Imm,  Ind,  Abs,  Abs,  Abs,
            /* 0x70 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
            /* 0x80 */ Imm, IndX, Imm, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Imp, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0x90 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpY, ZpY, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsY, AbsY,
            /* 0xA0 */ Imm, IndX, Imm, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Imp, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0xB0 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpY, ZpY, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsY, AbsY,
            /* 0xC0 */ Imm, IndX, Imm, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Imp, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0xD0 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
            /* 0xE0 */ Imm, IndX, Imm, IndX, Zp,  Zp,  Zp,  Zp,  Imp, Imm,  Imp, Imm,  Abs,  Abs,  Abs,  Abs,
            /* 0xF0 */ Rel, IndY, Imp, IndY, ZpX, ZpX, ZpX, ZpX, Imp, AbsY, Imp, AbsY, AbsX, AbsX, AbsX, AbsX,
        ];

        TABLE[self.0 as usize]
    }
}

/// CPU 命令のオペランド。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operand {
    Imp,
    Acc,
    Imm(u8),
    Zp(ZpAddress),
    ZpX(ZpAddress),
    ZpY(ZpAddress),
    Abs(Address),
    AbsX(Address),
    AbsY(Address),
    Ind(Address),
    IndX(ZpAddress),
    IndY(ZpAddress),
    Rel(i8),
}

impl Operand {
    /// オペランドのバイト数を返す。
    pub const fn len(self) -> usize {
        match self {
            Self::Imp => 0,
            Self::Acc => 0,
            Self::Imm(_) => 1,
            Self::Zp(_) => 1,
            Self::ZpX(_) => 1,
            Self::ZpY(_) => 1,
            Self::Abs(_) => 2,
            Self::AbsX(_) => 2,
            Self::AbsY(_) => 2,
            Self::Ind(_) => 2,
            Self::IndX(_) => 1,
            Self::IndY(_) => 1,
            Self::Rel(_) => 1,
        }
    }

    /// オペランドが 0 バイトかどうかを返す。
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// オペランドを機械語のバイト列に変換する。
    pub fn to_bytes(self) -> ArrayVec<u8, 2> {
        macro_rules! bytes {
            ($arg:expr) => {{
                ArrayVec::<u8, 2>::from_iter($arg.to_le_bytes())
            }};
        }

        match self {
            Self::Imp | Self::Acc => ArrayVec::<u8, 2>::new(),
            Self::Imm(imm) => bytes!(imm),
            Self::Zp(zp) => bytes!(zp),
            Self::ZpX(zp) => bytes!(zp),
            Self::ZpY(zp) => bytes!(zp),
            Self::Abs(abs) => bytes!(abs),
            Self::AbsX(abs) => bytes!(abs),
            Self::AbsY(abs) => bytes!(abs),
            Self::Ind(abs) => bytes!(abs),
            Self::IndX(zp) => bytes!(zp),
            Self::IndY(zp) => bytes!(zp),
            Self::Rel(rel) => bytes!(rel),
        }
    }
}

/// 命令を実行した後のプログラムカウンタの動き。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpSucc {
    /// 非制御フロー命令。オフセット(命令長)を保持する。
    Normal(NonZeroUsize),
    /// brk 命令。
    Brk,
    /// 全ての kil 命令。
    Kil,
    /// 全ての分岐命令。分岐オフセットを保持する。
    Branch(i8),
    /// jsr 命令。飛び先のアドレスを保持する。
    Jsr(Address),
    /// rti 命令。
    Rti,
    /// rts 命令。
    Rts,
    /// 絶対 jmp 命令。飛び先のアドレスを保持する。
    JmpAbs(Address),
    /// 間接 jmp 命令。ポインタのアドレスを保持する。
    JmpInd(Address),
}

/// CPU 命令のアドレッシングモード。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Addressing {
    Imp,
    Acc,
    Imm,
    Zp,
    ZpX,
    ZpY,
    Abs,
    AbsX,
    AbsY,
    Ind,
    IndX,
    IndY,
    Rel,
}

impl Addressing {
    /// オペランドのバイト数を返す。
    const fn operand_len(self) -> usize {
        match self {
            Self::Imp => 0,
            Self::Acc => 0,
            Self::Imm => 1,
            Self::Zp => 1,
            Self::ZpX => 1,
            Self::ZpY => 1,
            Self::Abs => 2,
            Self::AbsX => 2,
            Self::AbsY => 2,
            Self::Ind => 2,
            Self::IndX => 1,
            Self::IndY => 1,
            Self::Rel => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_opcode() {
        let asl_zp = Opcode::new(0x06);
        let slo_zp = Opcode::new(0x07);
        let bpl = Opcode::new(0x10);
        let jsr = Opcode::new(0x20);
        let sta_zp = Opcode::new(0x85);
        let lda_abx = Opcode::new(0xBD);

        assert_eq!(asl_zp.operand_len(), 1);
        assert_eq!(slo_zp.operand_len(), 1);
        assert_eq!(bpl.operand_len(), 1);
        assert_eq!(jsr.operand_len(), 2);
        assert_eq!(sta_zp.operand_len(), 1);
        assert_eq!(lda_abx.operand_len(), 2);

        assert!(asl_zp.is_official());
        assert!(!slo_zp.is_official());
        assert!(bpl.is_official());
        assert!(jsr.is_official());
        assert!(sta_zp.is_official());
        assert!(lda_abx.is_official());

        assert!(asl_zp.is_read());
        assert!(slo_zp.is_read());
        assert!(!bpl.is_read());
        assert!(!jsr.is_read());
        assert!(!sta_zp.is_read());
        assert!(lda_abx.is_read());

        assert!(asl_zp.is_write());
        assert!(slo_zp.is_write());
        assert!(!bpl.is_write());
        assert!(!jsr.is_write());
        assert!(sta_zp.is_write());
        assert!(!lda_abx.is_write());

        assert!(!asl_zp.is_branch());
        assert!(!slo_zp.is_branch());
        assert!(bpl.is_branch());
        assert!(!jsr.is_branch());
        assert!(!sta_zp.is_branch());
        assert!(!lda_abx.is_branch());

        assert!(!asl_zp.is_kil());
        assert!(!slo_zp.is_kil());
        assert!(!bpl.is_kil());
        assert!(!jsr.is_kil());
        assert!(!sta_zp.is_kil());
        assert!(!lda_abx.is_kil());
        assert!(Opcode::new(0x02).is_kil());

        assert!(!asl_zp.is_flow());
        assert!(!slo_zp.is_flow());
        assert!(bpl.is_flow());
        assert!(jsr.is_flow());
        assert!(!sta_zp.is_flow());
        assert!(!lda_abx.is_flow());
        assert!(Opcode::new(0x02).is_flow());
    }

    #[test]
    fn test_operand() {
        assert_equal(Operand::Imp.to_bytes(), []);
        assert_equal(Operand::Acc.to_bytes(), []);
        assert_equal(Operand::Zp(ZpAddress::new(0xFF)).to_bytes(), [0xFF]);
        assert_equal(Operand::ZpX(ZpAddress::new(0xFF)).to_bytes(), [0xFF]);
        assert_equal(Operand::ZpY(ZpAddress::new(0xFF)).to_bytes(), [0xFF]);
        assert_equal(Operand::Abs(Address::new(0xCDAB)).to_bytes(), [0xAB, 0xCD]);
        assert_equal(Operand::AbsX(Address::new(0xCDAB)).to_bytes(), [0xAB, 0xCD]);
        assert_equal(Operand::AbsY(Address::new(0xCDAB)).to_bytes(), [0xAB, 0xCD]);
        assert_equal(Operand::Ind(Address::new(0xCDAB)).to_bytes(), [0xAB, 0xCD]);
        assert_equal(Operand::IndX(ZpAddress::new(0xFF)).to_bytes(), [0xFF]);
        assert_equal(Operand::IndY(ZpAddress::new(0xFF)).to_bytes(), [0xFF]);
        assert_equal(Operand::Rel(0).to_bytes(), [0]);
        assert_equal(Operand::Rel(1).to_bytes(), [1]);
        assert_equal(Operand::Rel(-1).to_bytes(), [0xFF]);
    }

    #[test]
    fn test_op_fetch() {
        assert_eq!(Op::fetch(&[0x08]), Op::Php);
        assert_eq!(Op::fetch(&[0x0A]), Op::AslAcc);
        assert_eq!(Op::fetch(&[0x09, 0xFF]), Op::OraImm(0xFF));
        assert_eq!(Op::fetch(&[0x05, 0xFF]), Op::OraZp(ZpAddress::new(0xFF)));
        assert_eq!(Op::fetch(&[0x15, 0xFF]), Op::OraZpX(ZpAddress::new(0xFF)));
        assert_eq!(Op::fetch(&[0xB6, 0xFF]), Op::LdxZpY(ZpAddress::new(0xFF)));
        assert_eq!(
            Op::fetch(&[0x0D, 0xAB, 0xCD]),
            Op::OraAbs(Address::new(0xCDAB))
        );
        assert_eq!(
            Op::fetch(&[0x1D, 0xAB, 0xCD]),
            Op::OraAbsX(Address::new(0xCDAB))
        );
        assert_eq!(
            Op::fetch(&[0x19, 0xAB, 0xCD]),
            Op::OraAbsY(Address::new(0xCDAB))
        );
        assert_eq!(
            Op::fetch(&[0x6C, 0xAB, 0xCD]),
            Op::JmpInd(Address::new(0xCDAB))
        );
        assert_eq!(Op::fetch(&[0x01, 0xFF]), Op::OraIndX(ZpAddress::new(0xFF)));
        assert_eq!(Op::fetch(&[0x11, 0xFF]), Op::OraIndY(ZpAddress::new(0xFF)));
        assert_eq!(Op::fetch(&[0x10, 0]), Op::Bpl(0));
        assert_eq!(Op::fetch(&[0x10, 1]), Op::Bpl(1));
        assert_eq!(Op::fetch(&[0x10, 0xFF]), Op::Bpl(-1));
    }

    #[test]
    #[should_panic]
    fn test_op_fetch_empty() {
        let _ = Op::fetch(&[]);
    }

    #[test]
    #[should_panic]
    fn test_op_fetch_tooshort() {
        let _ = Op::fetch(&[0xA9]);
    }

    #[test]
    #[should_panic]
    fn test_op_fetch_toolong() {
        let _ = Op::fetch(&[0xA9, 0, 0]);
    }
}
