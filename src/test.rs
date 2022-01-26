use crate::opc::{AddressMode, Inst};
use crate::util::*;

#[test]
pub fn test_ora() {
    let mut cpu = new_cpu_empty_mem();
    cpu.set_ac(0x00);
    cpu.write_to_mem(0x01, 0x00);
    cpu.execute_inst(Inst::Ora, AddressMode::Imm).unwrap();
    assert!(cpu.z_flag());
    assert!(!cpu.n_flag());
    cpu.write_to_mem(0x03, 0b10011100);
    cpu.execute_inst(Inst::Ora, AddressMode::Imm).unwrap();
    assert!(!cpu.z_flag());
    assert!(cpu.n_flag());
}
