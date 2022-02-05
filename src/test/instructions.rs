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
    cpu.step_inst(Inst::And, AddressMode::ABS).unwrap();
    assert_eq!(cpu.ac(), 0x48);
}
