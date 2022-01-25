use crate::cpu::Cpu;
use crate::memory::SimpleMemory;
use asm6502::assemble;

pub fn new_cpu_with_asm(asm: &str) -> Result<Cpu<SimpleMemory>, String> {
    let mut bin = Vec::new();
    assemble(asm.as_bytes(), &mut bin)?;
    let mem = SimpleMemory::from_rom(&bin);
    let cpu = Cpu::with_mem(mem);
    Ok(cpu)
}

#[test]
pub fn test_ora() {
    // Test immediate addressing
    let asm = "ORA #$10\n";
    let mut cpu = new_cpu_with_asm(asm).unwrap();
    cpu.set_ac(0x03);
    let inst = cpu.fetch_next_inst();
    cpu.execute_inst(inst).unwrap();
    assert_eq!(cpu.ac(), 0x13);
}
