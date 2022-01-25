const OPS: [Inst; 0x10] = [
    // 0x00
    Inst::Brk(AddressMode::Impl),
    Inst::Ora(AddressMode::IndX),
    Inst::None,
    Inst::None,
    Inst::None,
    Inst::Ora(AddressMode::Zpg),
    Inst::Asl(AddressMode::Zpg),
    Inst::None,
    Inst::Php(AddressMode::Impl),
    Inst::Ora(AddressMode::Imm),
    Inst::Asl(AddressMode::Acc),
    Inst::None,
    Inst::None,
    Inst::Ora(AddressMode::Abs),
    Inst::Asl(AddressMode::Abs),
    Inst::None,
    // 0x10
];

pub const fn get_inst(opc: u8) -> Inst {
    OPS[opc as usize]
}

#[derive(Copy, Clone)]
pub enum Inst {
    Adc(AddressMode),
    And(AddressMode),
    Asl(AddressMode),
    Bcc(AddressMode),
    Bcs(AddressMode),
    Beq(AddressMode),
    Bit(AddressMode),
    Bmi(AddressMode),
    Bne(AddressMode),
    Bpl(AddressMode),
    Brk(AddressMode),
    Bvc(AddressMode),
    Bvs(AddressMode),
    Clc(AddressMode),
    Cld(AddressMode),
    Cli(AddressMode),
    Clv(AddressMode),
    Cmp(AddressMode),
    Cpx(AddressMode),
    Cpy(AddressMode),
    Dec(AddressMode),
    Dex(AddressMode),
    Dey(AddressMode),
    Eor(AddressMode),
    Inc(AddressMode),
    Inx(AddressMode),
    Iny(AddressMode),
    Jmp(AddressMode),
    Jsr(AddressMode),
    Lda(AddressMode),
    Ldx(AddressMode),
    Ldy(AddressMode),
    Lsr(AddressMode),
    Nop(AddressMode),
    Ora(AddressMode),
    Pha(AddressMode),
    Php(AddressMode),
    Pla(AddressMode),
    Plp(AddressMode),
    Rol(AddressMode),
    Ror(AddressMode),
    Rti(AddressMode),
    Rts(AddressMode),
    Sbc(AddressMode),
    Sec(AddressMode),
    Sed(AddressMode),
    Sei(AddressMode),
    Sta(AddressMode),
    Stx(AddressMode),
    Sty(AddressMode),
    Tax(AddressMode),
    Tay(AddressMode),
    Tsx(AddressMode),
    Txa(AddressMode),
    Txs(AddressMode),
    Tya(AddressMode),
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
        }
    }
}

pub enum Operand {
    One(u8),
    Two(u8),
}
