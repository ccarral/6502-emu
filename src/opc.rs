#[derive(Copy, Clone)]
pub struct OpMode(pub Inst, pub AddressMode, pub u8);

pub const fn init_opc_array() -> [Option<OpMode>; 0xFF] {
    let mut opc_arr: [Option<OpMode>; 0xFF] = [None; 0xFF];
    opc_arr[0x00] = Some(OpMode(Inst::BRK, AddressMode::IMPL, 0));
    opc_arr[0x01] = Some(OpMode(Inst::Ora, AddressMode::INDX, 6));
    opc_arr[0x05] = Some(OpMode(Inst::Ora, AddressMode::ZPG, 3));
    opc_arr[0x06] = Some(OpMode(Inst::ASL, AddressMode::ZPG, 0));
    opc_arr[0x08] = Some(OpMode(Inst::Php, AddressMode::IMPL, 0));
    opc_arr[0x09] = Some(OpMode(Inst::Ora, AddressMode::IMM, 2));
    opc_arr[0x0A] = Some(OpMode(Inst::ASL, AddressMode::ACC, 0));
    opc_arr[0x0B] = None;
    opc_arr[0x0D] = Some(OpMode(Inst::Ora, AddressMode::ABS, 4));
    opc_arr[0x0E] = Some(OpMode(Inst::ASL, AddressMode::ABS, 0));

    opc_arr
}

pub const fn get_op_mode(opc: u8) -> Option<OpMode> {
    OPS[opc as usize]
}

#[derive(Copy, Clone)]
pub enum Inst {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
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
    RTI,
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
}

#[derive(Copy, Clone)]
pub enum AddressMode {
    ACC,
    ABS,
    ABSX,
    ABSY,
    IMM,
    IMPL,
    IND,
    INDX,
    INDY,
    REL,
    ZPG,
    ZPGX,
    ZPGY,
}
