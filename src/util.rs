use crate::cpu::Cpu;
use crate::memory::SimpleMemory;
use asm6502::assemble;

pub fn combine_u8(ll: u8, hh: u8) -> u16 {
    let low = (ll as u16) << 8;
    let high = hh as u16;
    low | high
}

pub fn new_mem_with_asm(asm: &str) -> Result<SimpleMemory, String> {
    let mut bin = Vec::new();
    assemble(asm.as_bytes(), &mut bin)?;
    Ok(SimpleMemory::from_rom(&bin))
}

pub fn new_cpu_with_asm(asm: &str) -> Result<Cpu<SimpleMemory>, String> {
    let mem = new_mem_with_asm(asm)?;
    let cpu = Cpu::with_mem(mem);
    Ok(cpu)
}

pub fn new_cpu_empty_mem() -> Cpu<SimpleMemory> {
    let mem = SimpleMemory::from_rom(&[]);
    let cpu = Cpu::with_mem(mem);
    cpu
}
