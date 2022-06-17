use crate::opc::AddressMode;
use crate::util;

#[test]
fn test_absolute_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.write_to_mem(0x0601, 0x69);
    cpu.write_to_mem(0x0602, 0x99);
    let effective_addr = cpu.get_effective_address(&AddressMode::ABS);
    assert_eq!(effective_addr, 0x9969);
}

#[test]
fn test_absolute_x_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0F01);
    cpu.write_to_mem(0x0F02, 0x69);
    cpu.write_to_mem(0x0F03, 0x42);
    cpu.set_x(0xF);
    let effective_addr = cpu.get_effective_address(&AddressMode::ABSX);
    assert_eq!(effective_addr, 0x4278);
}

#[test]
fn test_absolute_y_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0010);
    cpu.write_to_mem(0x0011, 0x55);
    cpu.write_to_mem(0x0012, 0x66);
    cpu.set_y(0xF);
    let effective_addr = cpu.get_effective_address(&AddressMode::ABSY);
    assert_eq!(effective_addr, 0x6664);
}

#[test]
fn test_indirect_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x13FE);
    cpu.write_to_mem(0xCC01, 0xCA);
    cpu.write_to_mem(0xCC02, 0xAC);
    cpu.write_to_mem(0x13FF, 0x01);
    cpu.write_to_mem(0x1300, 0xCC);
    let effective_addr = cpu.get_effective_address(&AddressMode::IND);
    assert_eq!(effective_addr, 0xACCA);
}

#[test]
fn test_indirect_x_indexed_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.set_x(0x01);
    cpu.write_to_mem(0x0601, 0x00);
    cpu.write_to_mem(0x0001, 0x05);
    cpu.write_to_mem(0x0002, 0x07);
    let effective_addr = cpu.get_effective_address(&AddressMode::INDX);
    assert_eq!(effective_addr, 0x0705);
}

#[test]
fn test_indirect_y_indexed_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x00);
    cpu.write_to_mem(0x01, 0x86);
    cpu.set_y(0x10);
    cpu.write_to_mem(0x0086, 0x28);
    cpu.write_to_mem(0x0087, 0x40);
    let effective_addr = cpu.get_effective_address(&AddressMode::INDY);
    assert_eq!(effective_addr, 0x4038);
}

#[test]
fn test_relative_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0500);
    // Offset de -128 (dec)
    cpu.write_to_mem(0x0501, 0x80);
    let target_addr = cpu.get_relative_address(cpu.pc() + 1);
    assert_eq!(target_addr, 0x0482);

    cpu.set_pc(0x0600);
    // Offset de -1 (dec)
    cpu.write_to_mem(0x0601, 0xFF);
    let target_addr = cpu.get_relative_address(cpu.pc() + 1);
    assert_eq!(target_addr, 0x0601);

    cpu.set_pc(0x0400);
    // Offset de 127
    cpu.write_to_mem(0x0401, 0x7F);
    let target_addr = cpu.get_relative_address(cpu.pc() + 1);
    assert_eq!(target_addr, 0x481);

    cpu.set_pc(0x0900);
    // Offset de 0
    cpu.write_to_mem(0x0901, 0x00);
    let target_addr = cpu.get_relative_address(cpu.pc() + 1);
    assert_eq!(target_addr, 0x902);
}

#[test]
fn test_zpg_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.write_to_mem(0x0601, 0x18);
    let target_addr = cpu.get_effective_address(&AddressMode::ZPGX);
    assert_eq!(target_addr, 0x0018);
}

#[test]
fn test_zpg_x_indexed_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.set_x(0x11);
    cpu.write_to_mem(0x0601, 0x18);
    let target_addr = cpu.get_effective_address(&AddressMode::ZPGX);
    assert_eq!(target_addr, 0x0029);
}

#[test]
fn test_zpg_y_indexed_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.set_y(0x11);
    cpu.write_to_mem(0x0601, 0x18);
    let target_addr = cpu.get_effective_address(&AddressMode::ZPGY);
    assert_eq!(target_addr, 0x0029);
}
