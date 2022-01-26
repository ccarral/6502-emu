use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::opc::OpMode;
use crate::util::*;

#[test]
pub fn test_ora() {
    // Test immediate addressing
    let asm = "ORA #$10\n";
    let mut cpu = new_cpu_with_asm(asm).unwrap();
    cpu.set_ac(0x03);
    let OpMode(inst, addr_mode, _cycles) = cpu.fetch_next_inst().unwrap();
    cpu.execute_inst(inst, addr_mode).unwrap();
    assert_eq!(cpu.ac(), 0x13);

    // Test Zpg addressing
    let mut mem = new_mem_with_asm("ORA $20").unwrap();
    mem.write_byte(0x20, 0x10);
    let mut cpu = Cpu::with_mem(mem);
    cpu.set_ac(0x03);
    let OpMode(inst, addr_mode, _cycles) = cpu.fetch_next_inst().unwrap();
    cpu.execute_inst(inst, addr_mode).unwrap();
    assert_eq!(cpu.ac(), 0x13);
}
