use crate::opc::AddressMode;
use crate::util;

#[test]
fn test_absolute_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0600);
    cpu.write_to_mem(0x0601, 0x69);
    cpu.write_to_mem(0x0602, 0x99);
    let effective_addr = cpu.get_effective_address(&AddressMode::Abs);
    assert_eq!(effective_addr, 0x9969);
}

#[test]
fn test_absolute_x_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0F01);
    cpu.write_to_mem(0x0F02, 0x69);
    cpu.write_to_mem(0x0F03, 0x42);
    cpu.set_x(0xF);
    let effective_addr = cpu.get_effective_address(&AddressMode::AbsX);
    assert_eq!(effective_addr, 0x4278);
}

#[test]
fn test_absolute_y_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x0010);
    cpu.write_to_mem(0x0011, 0x55);
    cpu.write_to_mem(0x0012, 0x66);
    cpu.set_y(0xF);
    let effective_addr = cpu.get_effective_address(&AddressMode::AbsY);
    assert_eq!(effective_addr, 0x6664);
}

#[test]
fn test_indirect_addressing() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.set_pc(0x13FE);
    cpu.write_to_mem(0x13FF, 0x01);
    cpu.write_to_mem(0x1300, 0xCC);
    let effective_addr = cpu.get_effective_address(&AddressMode::Ind);
    assert_eq!(effective_addr, 0xCC01);
}
