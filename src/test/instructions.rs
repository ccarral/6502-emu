use crate::opc::{AddressMode, Inst};
use crate::util;

#[test]
pub fn test_adc() {
    let mut cpu = util::new_cpu_empty_mem();

    cpu.write_to_mem(0x0001, 0x69);
    cpu.step_inst(Inst::ADC, AddressMode::IMM).unwrap();
    assert_eq!(cpu.ac(), 0x69);

    cpu.write_to_mem(0x03, 0x63);
    cpu.write_to_mem(0x0063, 0x42);
    cpu.step_inst(Inst::ADC, AddressMode::ZPG).unwrap();
    assert_eq!(cpu.ac(), 0xAB);

    cpu.set_ac(0);

    cpu.write_to_mem(0x0005, 0x33);
    cpu.write_to_mem(0x003A, 0x50);
    cpu.set_x(0x07);
    cpu.step_inst(Inst::ADC, AddressMode::ZPGX).unwrap();
    assert_eq!(cpu.ac(), 0x50);

    cpu.set_ac(0);
    cpu.write_to_mem(0x0007, 0xFF);
    cpu.set_y(0x01);
    cpu.step_inst(Inst::ADC, AddressMode::ZPGY).unwrap();
    assert_eq!(cpu.ac(), 0x00);
    assert!(cpu.z_flag());
}

#[test]
pub fn test_and() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_ac(0x4A);
    cpu.write_to_mem(0x0001, 0x56);
    cpu.write_to_mem(0x0002, 0x44);
    cpu.write_to_mem(0x4456, 0x48);
    cpu.step_inst(Inst::AND, AddressMode::ABS).unwrap();
    assert_eq!(cpu.ac(), 0x48);
}

#[test]
pub fn test_asl() {
    let mut cpu = util::new_cpu_empty_mem();
    // Test with acc
    cpu.set_ac(0b0100_0000);
    cpu.step_inst(Inst::ASL, AddressMode::ACC).unwrap();
    assert_eq!(cpu.ac(), 0b1000_0000);
    assert!(!cpu.c_flag());
    cpu.step_inst(Inst::ASL, AddressMode::ACC).unwrap();
    assert!(cpu.c_flag());

    // Test with memory
    cpu.set_pc(0x0600);
    cpu.write_to_mem(0x0040, 0b0010_0000);
    cpu.write_to_mem(0x0601, 0x40);
    cpu.step_inst(Inst::ASL, AddressMode::ZPG).unwrap();
    assert_eq!(cpu.read_byte_from_mem(0x0040), 0b0100_0000);
}

#[test]
pub fn test_branch_instructions() {
    let mut cpu = util::new_cpu_empty_mem();

    // BCC
    cpu.write_c_flag(true);
    cpu.set_pc(0x0200);
    cpu.step_inst(Inst::BCC, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0202);
    cpu.write_c_flag(false);
    // -30
    cpu.write_to_mem(0x0203, 0xE2);
    cpu.step_inst(Inst::BCC, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x01E6);

    // BCS
    cpu.write_c_flag(false);
    cpu.set_pc(0x0300);
    cpu.step_inst(Inst::BCS, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0302);
    cpu.write_c_flag(true);
    // -30
    cpu.write_to_mem(0x0303, 0xE2);
    cpu.step_inst(Inst::BCS, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x02E6);

    // BEQ
    cpu.write_z_flag(false);
    cpu.set_pc(0x0400);
    cpu.step_inst(Inst::BEQ, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0402);
    cpu.write_z_flag(true);
    // -30
    cpu.write_to_mem(0x0403, 0xE2);
    cpu.step_inst(Inst::BEQ, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x03E6);

    // BMI
    cpu.update_n_flag_with(0b11000000);
    cpu.set_pc(0x0500);
    cpu.step_inst(Inst::BEQ, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0502);
    cpu.update_n_flag_with(2);
    // -30
    cpu.write_to_mem(0x0503, 0xE2);
    cpu.step_inst(Inst::BEQ, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x04E6);

    // BNE
    cpu.write_z_flag(true);
    cpu.set_pc(0x0600);
    cpu.step_inst(Inst::BNE, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0602);
    cpu.write_z_flag(false);
    // -30
    cpu.write_to_mem(0x0603, 0xE2);
    cpu.step_inst(Inst::BNE, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x05E6);

    // BPL
    cpu.update_n_flag_with(0b11111111);
    cpu.set_pc(0x0700);
    cpu.step_inst(Inst::BPL, AddressMode::REL).unwrap();
    // No jump
    assert_eq!(cpu.pc(), 0x0702);
    cpu.update_n_flag_with(0b00000001);
    // -30
    cpu.write_to_mem(0x0703, 0xE2);
    cpu.step_inst(Inst::BPL, AddressMode::REL).unwrap();
    assert_eq!(cpu.pc(), 0x06E6);
}

#[test]
pub fn test_bit() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.write_to_mem(0x0500, 0x69);
    cpu.set_ac(0x69);
    cpu.write_to_mem(0x0001, 0x00);
    cpu.write_to_mem(0x0002, 0x05);
    cpu.step_inst(Inst::BIT, AddressMode::ABS).unwrap();
    // Equal, so Z = 0
    assert!(!cpu.z_flag());

    cpu.set_ac(0x87);
    cpu.write_to_mem(0x0040, 0x88);
    cpu.write_to_mem(0x0004, 0x40);
    cpu.step_inst(Inst::BIT, AddressMode::ZPG).unwrap();
    // Different, so Z = 1
    assert!(cpu.z_flag());
}

#[test]
pub fn test_brk() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0100);
    cpu.write_n_flag(true);
    cpu.write_to_mem(0xFFFE, 0x69);
    cpu.write_to_mem(0xFFFF, 0x42);
    cpu.step_inst(Inst::BRK, AddressMode::IMPL).unwrap();
    assert_eq!(cpu.pc(), 0x4269);

    cpu.write_n_flag(false);
    cpu.write_c_flag(true);
    cpu.write_v_flag(true);
    cpu.write_b_flag(false);
    cpu.write_z_flag(true);
    cpu.write_d_flag(true);

    cpu.step_inst(Inst::RTI, AddressMode::IMPL).unwrap();
    assert_eq!(cpu.pc(), 0x0100 + 2);
    assert_eq!(cpu.b_flag(), true);
    assert_eq!(cpu.n_flag(), true);
}
