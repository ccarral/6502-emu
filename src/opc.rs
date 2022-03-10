#[derive(Copy, Clone)]
// (Instruction, AddressMode, cycles)
pub struct OpMode(pub Inst, pub AddressMode, pub u8);

pub fn init_opc_array() -> [Option<OpMode>; 0xFF] {
    let mut opc_arr: [Option<OpMode>; 0xFF] = [None; 0xFF];
    let mut already_set: [bool; 0xFF] = [false; 0xFF];

    let mut add_to_opc_arr = |opc: usize, inst: Inst, addr_mode: AddressMode, cycles: u8| {
        if already_set[opc] {
            panic!("opc previously set. Please check opc.")
        } else {
            opc_arr[opc] = Some(OpMode(inst, addr_mode, cycles));
            already_set[opc] = true;
        }
    };

    add_to_opc_arr(0x69, Inst::ADC, AddressMode::IMM, 2);
    add_to_opc_arr(0x65, Inst::ADC, AddressMode::ZPG, 3);
    add_to_opc_arr(0x75, Inst::ADC, AddressMode::ZPGX, 4);
    add_to_opc_arr(0x6D, Inst::ADC, AddressMode::ABS, 4);
    add_to_opc_arr(0x7D, Inst::ADC, AddressMode::ABSX, 4);
    add_to_opc_arr(0x79, Inst::ADC, AddressMode::ABSY, 4);
    add_to_opc_arr(0x61, Inst::ADC, AddressMode::INDX, 6);
    add_to_opc_arr(0x71, Inst::ADC, AddressMode::INDY, 5);

    add_to_opc_arr(0x29, Inst::AND, AddressMode::IMM, 2);
    add_to_opc_arr(0x25, Inst::AND, AddressMode::ZPG, 3);
    add_to_opc_arr(0x35, Inst::AND, AddressMode::ZPGX, 4);
    add_to_opc_arr(0x2D, Inst::AND, AddressMode::ABS, 4);
    add_to_opc_arr(0x3D, Inst::AND, AddressMode::ABSX, 4);
    add_to_opc_arr(0x39, Inst::AND, AddressMode::ABSY, 4);
    add_to_opc_arr(0x21, Inst::AND, AddressMode::INDX, 6);
    add_to_opc_arr(0x31, Inst::AND, AddressMode::INDY, 5);

    add_to_opc_arr(0x0A, Inst::ASL, AddressMode::ACC, 2);
    add_to_opc_arr(0x06, Inst::ASL, AddressMode::ZPG, 5);
    add_to_opc_arr(0x16, Inst::ASL, AddressMode::ZPGX, 6);
    add_to_opc_arr(0x0E, Inst::ASL, AddressMode::ABS, 6);
    add_to_opc_arr(0x1E, Inst::ASL, AddressMode::ABSX, 7);

    add_to_opc_arr(0x90, Inst::BCC, AddressMode::REL, 2);

    add_to_opc_arr(0xB0, Inst::BCS, AddressMode::REL, 2);

    add_to_opc_arr(0xF0, Inst::BEQ, AddressMode::REL, 2);

    add_to_opc_arr(0x24, Inst::BIT, AddressMode::ZPG, 3);
    add_to_opc_arr(0x2C, Inst::BIT, AddressMode::ABS, 3);

    add_to_opc_arr(0x30, Inst::BMI, AddressMode::REL, 2);

    add_to_opc_arr(0xD0, Inst::BNE, AddressMode::REL, 2);

    add_to_opc_arr(0x10, Inst::BPL, AddressMode::REL, 2);

    add_to_opc_arr(0x00, Inst::BRK, AddressMode::IMPL, 7);

    add_to_opc_arr(0x50, Inst::BVC, AddressMode::REL, 2);

    add_to_opc_arr(0x70, Inst::BVS, AddressMode::REL, 2);

    add_to_opc_arr(0x18, Inst::CLC, AddressMode::IMPL, 2);

    add_to_opc_arr(0xD8, Inst::CLD, AddressMode::IMPL, 2);

    add_to_opc_arr(0x58, Inst::CLI, AddressMode::IMPL, 2);

    add_to_opc_arr(0xB8, Inst::CLV, AddressMode::IMPL, 2);

    add_to_opc_arr(0xC9, Inst::CMP, AddressMode::IMM, 2);
    add_to_opc_arr(0xC5, Inst::CMP, AddressMode::ZPG, 3);
    add_to_opc_arr(0xD5, Inst::CMP, AddressMode::ZPGX, 4);
    add_to_opc_arr(0xCD, Inst::CMP, AddressMode::ABS, 4);
    add_to_opc_arr(0xDD, Inst::CMP, AddressMode::ABSX, 4);
    add_to_opc_arr(0xD9, Inst::CMP, AddressMode::ABSY, 4);
    add_to_opc_arr(0xC1, Inst::CMP, AddressMode::INDX, 6);
    add_to_opc_arr(0xD1, Inst::CMP, AddressMode::INDY, 5);

    add_to_opc_arr(0xE0, Inst::CPX, AddressMode::IMM, 2);
    add_to_opc_arr(0xE4, Inst::CPX, AddressMode::ZPG, 3);
    add_to_opc_arr(0xEC, Inst::CPX, AddressMode::ABS, 4);

    add_to_opc_arr(0xC0, Inst::CPY, AddressMode::IMM, 2);
    add_to_opc_arr(0xC4, Inst::CPY, AddressMode::ZPG, 3);
    add_to_opc_arr(0xCC, Inst::CPY, AddressMode::ABS, 4);

    add_to_opc_arr(0xC6, Inst::DEC, AddressMode::ZPG, 5);
    add_to_opc_arr(0xD6, Inst::DEC, AddressMode::ZPGX, 6);
    add_to_opc_arr(0xCE, Inst::DEC, AddressMode::ABS, 6);
    add_to_opc_arr(0xDE, Inst::DEC, AddressMode::ABSX, 7);

    add_to_opc_arr(0xCA, Inst::DEX, AddressMode::IMPL, 2);

    add_to_opc_arr(0x88, Inst::DEY, AddressMode::IMPL, 2);

    add_to_opc_arr(0x4A, Inst::LSR, AddressMode::ACC, 2);
    add_to_opc_arr(0x46, Inst::LSR, AddressMode::ZPG, 5);
    add_to_opc_arr(0x56, Inst::LSR, AddressMode::ZPGX, 6);
    add_to_opc_arr(0x4E, Inst::LSR, AddressMode::ABS, 6);
    add_to_opc_arr(0x5E, Inst::LSR, AddressMode::ABSX, 7);

    opc_arr
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
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    RTI,
    RTS,
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
