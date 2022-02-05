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
}
