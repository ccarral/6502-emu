use crate::opc::{AddressMode, Inst};
use crate::util::*;

#[test]
pub fn test_ora() {
    let mut cpu = new_cpu_empty_mem();
    cpu.set_ac(0x00);
    cpu.write_to_mem(0x0001, 0x00);
    cpu.execute_inst(Inst::Ora, AddressMode::Imm).unwrap();
    assert!(cpu.z_flag());
    assert!(!cpu.n_flag());
    cpu.write_to_mem(0x0003, 0b10011100);
    cpu.execute_inst(Inst::Ora, AddressMode::Imm).unwrap();
    assert!(!cpu.z_flag());
    assert!(cpu.n_flag());

    cpu.write_to_mem(0x00F0, 0x20);
    cpu.write_to_mem(0x0005, 0xF0);
    cpu.set_ac(0x02);
    cpu.execute_inst(Inst::Ora, AddressMode::Zpg).unwrap();
    // 20 | 2
    assert_eq!(cpu.ac(), 0x22);
    assert!(!cpu.z_flag());
    assert!(!cpu.n_flag());

    cpu.set_x(0x03);
    cpu.write_to_mem(0x0007, 0x11);
    cpu.write_to_mem(0x0014, 0x45);
    cpu.execute_inst(Inst::Ora, AddressMode::ZpgX).unwrap();
    assert_eq!(cpu.ac(), 0x67);
    assert!(!cpu.z_flag());
    assert!(!cpu.n_flag());

    cpu.set_ac(0x02);
    cpu.write_to_mem(0x80F9, 0x30);
    cpu.write_to_mem(0x0009, 0xF9);
    cpu.write_to_mem(0x000A, 0x80);
    cpu.execute_inst(Inst::Ora, AddressMode::Abs).unwrap();
    // 30 | 02
    assert_eq!(cpu.ac(), 0x32);
    assert!(!cpu.z_flag());
    assert!(!cpu.n_flag());

    cpu.set_ac(0x02);
    cpu.set_pc(0x08);
    cpu.set_x(0x0A);
    cpu.write_to_mem(0x8103, 0x30);
    cpu.write_to_mem(0x0009, 0xF9);
    cpu.write_to_mem(0x000A, 0x80);
    cpu.execute_inst(Inst::Ora, AddressMode::AbsX).unwrap();
    // 30 | 02
    assert_eq!(cpu.ac(), 0x32);
    assert!(!cpu.z_flag());
    assert!(!cpu.n_flag());

    cpu.reset_flags();

    cpu.set_ac(0x02);
    cpu.set_pc(0x08);
    cpu.set_y(0x0A);
    cpu.write_to_mem(0x8103, 0x30);
    cpu.write_to_mem(0x0009, 0xF9);
    cpu.write_to_mem(0x000A, 0x80);
    cpu.execute_inst(Inst::Ora, AddressMode::AbsY).unwrap();
    // 30 | 02
    assert_eq!(cpu.ac(), 0x32);
    assert!(!cpu.z_flag());
    assert!(!cpu.n_flag());
}
