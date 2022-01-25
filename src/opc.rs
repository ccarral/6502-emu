#[derive(Copy, Clone)]
pub struct OpMode(pub Inst, pub AddressMode, pub u8);

const OPS: [Option<OpMode>; 0x10] = [
    // 0x00
    Some(OpMode(Inst::Brk, AddressMode::Impl, 0)),
    Some(OpMode(Inst::Ora, AddressMode::IndX, 0)),
    None,
    None,
    None,
    Some(OpMode(Inst::Ora, AddressMode::Zpg, 0)),
    Some(OpMode(Inst::Asl, AddressMode::Zpg, 0)),
    None,
    Some(OpMode(Inst::Php, AddressMode::Impl, 0)),
    Some(OpMode(Inst::Ora, AddressMode::Imm, 0)),
    Some(OpMode(Inst::Asl, AddressMode::Acc, 0)),
    None,
    None,
    Some(OpMode(Inst::Ora, AddressMode::Abs, 0)),
    Some(OpMode(Inst::Asl, AddressMode::Abs, 0)),
    None,
    // 0x10
];

pub const fn get_op_mode(opc: u8) -> Option<OpMode> {
    OPS[opc as usize]
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
