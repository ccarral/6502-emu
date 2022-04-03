use std::collections::HashMap;

#[derive(Copy, Clone)]
// (Instruction, AddressMode, cycles)
pub struct OpMode(pub Inst, pub AddressMode, pub u8);

pub fn init_opc_array() -> [Option<OpMode>; 0xFF] {
    let mut opc_arr: [Option<OpMode>; 0xFF] = [None; 0xFF];
    let mut already_set: [bool; 0xFF] = [false; 0xFF];
    let mut address_mode_already_set = HashMap::new();

    // returns a bitmask according to addressing mode
    const fn addr_mode_bitmask(addr_mode: &AddressMode) -> u16 {
        match addr_mode {
            AddressMode::ACC => 1 << 0,
            AddressMode::ABS => 1 << 1,
            AddressMode::ABSX => 1 << 2,
            AddressMode::ABSY => 1 << 3,
            AddressMode::IMM => 1 << 4,
            AddressMode::IMPL => 1 << 5,
            AddressMode::IND => 1 << 6,
            AddressMode::INDX => 1 << 7,
            AddressMode::INDY => 1 << 8,
            AddressMode::REL => 1 << 9,
            AddressMode::ZPG => 1 << 10,
            AddressMode::ZPGX => 1 << 11,
            AddressMode::ZPGY => 1 << 12,
        }
    }

    let mut add_to_opc_arr = |opc: usize, inst: Inst, addr_mode: AddressMode, cycles: u8| {
        if already_set[opc] {
            panic!("opc {opc:#02x} previously set for {inst:#?}. Please check opc.")
        } else {
            // Check that this addressing mode has not been set previously for this instruction
            let checked_address_modes = address_mode_already_set.entry(inst).or_insert(0u16);
            let address_mode_bitmask = addr_mode_bitmask(&addr_mode);

            // Check if bitmask is set
            if address_mode_bitmask & *checked_address_modes != 0 {
                panic!(
                    "Address mode {addr_mode:#?} already set for inst {inst:#?}. Please check opc."
                );
            } else {
                *checked_address_modes |= address_mode_bitmask;
            }

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

    add_to_opc_arr(0x49, Inst::EOR, AddressMode::IMM, 2);
    add_to_opc_arr(0x45, Inst::EOR, AddressMode::ZPG, 3);
    add_to_opc_arr(0x55, Inst::EOR, AddressMode::ZPGX, 4);
    add_to_opc_arr(0x4D, Inst::EOR, AddressMode::ABS, 4);
    add_to_opc_arr(0x5D, Inst::EOR, AddressMode::ABSX, 4);
    add_to_opc_arr(0x59, Inst::EOR, AddressMode::ABSY, 4);
    add_to_opc_arr(0x41, Inst::EOR, AddressMode::INDX, 6);
    add_to_opc_arr(0x51, Inst::EOR, AddressMode::INDY, 5);

    add_to_opc_arr(0xE6, Inst::INC, AddressMode::ZPG, 5);
    add_to_opc_arr(0xF6, Inst::INC, AddressMode::ZPGX, 6);
    add_to_opc_arr(0xEE, Inst::INC, AddressMode::ABS, 6);
    add_to_opc_arr(0xFE, Inst::INC, AddressMode::ABSX, 7);

    add_to_opc_arr(0xE8, Inst::INX, AddressMode::IMPL, 2);

    add_to_opc_arr(0xC8, Inst::INY, AddressMode::IMPL, 2);

    add_to_opc_arr(0x4C, Inst::JMP, AddressMode::ABS, 3);
    add_to_opc_arr(0x6C, Inst::JMP, AddressMode::IND, 5);

    add_to_opc_arr(0x20, Inst::JSR, AddressMode::ABS, 6);

    add_to_opc_arr(0xA9, Inst::LDA, AddressMode::IMM, 2);
    add_to_opc_arr(0xA5, Inst::LDA, AddressMode::ZPG, 3);
    add_to_opc_arr(0xB5, Inst::LDA, AddressMode::ZPGX, 6);
    add_to_opc_arr(0xAD, Inst::LDA, AddressMode::ABS, 6);
    add_to_opc_arr(0xBD, Inst::LDA, AddressMode::ABSX, 6);
    add_to_opc_arr(0xB9, Inst::LDA, AddressMode::ABSY, 6);
    add_to_opc_arr(0xA1, Inst::LDA, AddressMode::INDX, 6);
    add_to_opc_arr(0xB1, Inst::LDA, AddressMode::INDY, 6);

    add_to_opc_arr(0x4A, Inst::LSR, AddressMode::ACC, 2);
    add_to_opc_arr(0x46, Inst::LSR, AddressMode::ZPG, 5);
    add_to_opc_arr(0x56, Inst::LSR, AddressMode::ZPGX, 6);
    add_to_opc_arr(0x4E, Inst::LSR, AddressMode::ABS, 6);
    add_to_opc_arr(0x5E, Inst::LSR, AddressMode::ABSX, 7);

    add_to_opc_arr(0xEA, Inst::NOP, AddressMode::IMPL, 2);

    add_to_opc_arr(0x09, Inst::ORA, AddressMode::IMM, 2);
    add_to_opc_arr(0x05, Inst::ORA, AddressMode::ZPG, 3);
    add_to_opc_arr(0x15, Inst::ORA, AddressMode::ZPGX, 4);
    add_to_opc_arr(0x0D, Inst::ORA, AddressMode::ABS, 4);
    add_to_opc_arr(0x1D, Inst::ORA, AddressMode::ABSX, 4);
    add_to_opc_arr(0x19, Inst::ORA, AddressMode::ABSY, 4);
    add_to_opc_arr(0x01, Inst::ORA, AddressMode::INDX, 6);
    add_to_opc_arr(0x11, Inst::ORA, AddressMode::INDY, 5);

    add_to_opc_arr(0x48, Inst::PHA, AddressMode::IMPL, 3);

    add_to_opc_arr(0x08, Inst::PHP, AddressMode::IMPL, 3);

    add_to_opc_arr(0x68, Inst::PLA, AddressMode::IMPL, 3);

    add_to_opc_arr(0x28, Inst::PLP, AddressMode::IMPL, 3);

    add_to_opc_arr(0x2A, Inst::ROL, AddressMode::ACC, 2);
    add_to_opc_arr(0x26, Inst::ROL, AddressMode::ZPG, 5);
    add_to_opc_arr(0x36, Inst::ROL, AddressMode::ZPGX, 6);
    add_to_opc_arr(0x2E, Inst::ROL, AddressMode::ABS, 6);
    add_to_opc_arr(0x3E, Inst::ROL, AddressMode::ABSX, 7);

    add_to_opc_arr(0x6A, Inst::ROR, AddressMode::ACC, 2);
    add_to_opc_arr(0x66, Inst::ROR, AddressMode::ZPG, 5);
    add_to_opc_arr(0x76, Inst::ROR, AddressMode::ZPGX, 6);
    add_to_opc_arr(0x6E, Inst::ROR, AddressMode::ABS, 6);
    add_to_opc_arr(0x7E, Inst::ROR, AddressMode::ABSX, 7);

    add_to_opc_arr(0x60, Inst::RTS, AddressMode::IMPL, 6);

    add_to_opc_arr(0xE9, Inst::SBC, AddressMode::IMM, 2);
    add_to_opc_arr(0xE5, Inst::SBC, AddressMode::ZPG, 3);
    add_to_opc_arr(0xF5, Inst::SBC, AddressMode::ZPGX, 4);
    add_to_opc_arr(0xED, Inst::SBC, AddressMode::ABS, 4);
    add_to_opc_arr(0xFD, Inst::SBC, AddressMode::ABSX, 4);
    add_to_opc_arr(0xF9, Inst::SBC, AddressMode::ABSY, 4);
    add_to_opc_arr(0xE1, Inst::SBC, AddressMode::INDX, 6);
    add_to_opc_arr(0xF1, Inst::SBC, AddressMode::INDY, 5);

    add_to_opc_arr(0x38, Inst::SEC, AddressMode::IMPL, 2);

    add_to_opc_arr(0xF8, Inst::SED, AddressMode::IMPL, 2);

    add_to_opc_arr(0x78, Inst::SEI, AddressMode::IMPL, 2);

    add_to_opc_arr(0x85, Inst::STA, AddressMode::ZPG, 3);
    add_to_opc_arr(0x95, Inst::STA, AddressMode::ZPGX, 4);
    add_to_opc_arr(0x8D, Inst::STA, AddressMode::ABS, 4);
    add_to_opc_arr(0x9D, Inst::STA, AddressMode::ABSX, 5);
    add_to_opc_arr(0x99, Inst::STA, AddressMode::ABSY, 5);
    add_to_opc_arr(0x81, Inst::STA, AddressMode::INDX, 6);
    add_to_opc_arr(0x91, Inst::STA, AddressMode::INDY, 6);

    add_to_opc_arr(0x86, Inst::STX, AddressMode::ZPG, 3);
    add_to_opc_arr(0x96, Inst::STX, AddressMode::ZPGY, 4);
    add_to_opc_arr(0x8E, Inst::STX, AddressMode::ABS, 4);

    add_to_opc_arr(0x84, Inst::STY, AddressMode::ZPG, 3);
    add_to_opc_arr(0x94, Inst::STY, AddressMode::ZPGY, 4);
    add_to_opc_arr(0x8C, Inst::STY, AddressMode::ABS, 4);

    add_to_opc_arr(0xAA, Inst::TAX, AddressMode::IMPL, 2);

    add_to_opc_arr(0xA8, Inst::TAY, AddressMode::IMPL, 2);

    add_to_opc_arr(0xBA, Inst::TSX, AddressMode::IMPL, 2);

    add_to_opc_arr(0x8A, Inst::TXA, AddressMode::IMPL, 2);

    add_to_opc_arr(0x9A, Inst::TXS, AddressMode::IMPL, 2);

    add_to_opc_arr(0x98, Inst::TYA, AddressMode::IMPL, 2);

    opc_arr
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Copy, Clone, Debug)]
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
