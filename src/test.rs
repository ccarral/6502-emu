use crate::cpu::Cpu;
use crate::memory::{Memory, SimpleMemory};
use crate::opc::OpMode;
use asm6502::assemble;

fn new_mem_with_asm(asm: &str) -> Result<SimpleMemory, String> {
    let mut bin = Vec::new();
    assemble(asm.as_bytes(), &mut bin)?;
    Ok(SimpleMemory::from_rom(&bin))
}

fn new_cpu_with_asm(asm: &str) -> Result<Cpu<SimpleMemory>, String> {
    let mem = new_mem_with_asm(asm)?;
    let cpu = Cpu::with_mem(mem);
    Ok(cpu)
}

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
