const OPS: [Inst; 0x10] = [
    // 0x00
    Inst::Brk,
    Inst::Ora,
    Inst::None,
    Inst::None,
    Inst::None,
    Inst::Ora,
    Inst::Asl,
    Inst::None,
    Inst::Php,
    Inst::Ora,
    Inst::Asl,
    Inst::None,
    Inst::None,
    Inst::Ora,
    Inst::Asl,
    Inst::None,
    // 0x10
];

const ADDR_MODE: [AddressMode; 0x10] = [
    // 0x00
    AddressMode::Impl,
    AddressMode::IndX,
    AddressMode::None,
    AddressMode::None,
    AddressMode::None,
    AddressMode::Zpg,
    AddressMode::Zpg,
    AddressMode::None,
    AddressMode::Impl,
    AddressMode::Imm,
    AddressMode::Acc,
    AddressMode::None,
    AddressMode::None,
    AddressMode::Abs,
    AddressMode::Abs,
    AddressMode::None,
    // 0x10
];

pub const fn get_inst(opc: u8) -> Inst {
    OPS[opc as usize]
}

pub const fn get_address_mode(opc: u8) -> AddressMode {
    ADDR_MODE[opc as usize]
}

#[derive(Copy, Clone)]
pub enum Inst {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    None,
}

#[derive(Copy, Clone)]
pub enum AddressMode {
    Acc,
    Abs,
    AbsX,
    AbsY,
    Imm,
    Impl,
    Ind,
    IndX,
    IndY,
    Rel,
    Zpg,
    ZpgX,
    ZpgY,
    None,
}

impl AddressMode {
    pub const fn expected_bytes(&self) -> usize {
        match self {
            AddressMode::Acc => 0,
            AddressMode::Abs => 2,
            AddressMode::AbsX => 2,
            AddressMode::AbsY => 2,
            AddressMode::Imm => 1,
            AddressMode::Impl => 0,
            AddressMode::Ind => 2,
            AddressMode::IndX => 1,
            AddressMode::IndY => 1,
            AddressMode::Rel => 1,
            AddressMode::Zpg => 1,
            AddressMode::ZpgX => 1,
            AddressMode::ZpgY => 1,
            AddressMode::None => 0,
        }
    }
}

pub enum Operand {
    One(u8),
    Two(u8),
}
