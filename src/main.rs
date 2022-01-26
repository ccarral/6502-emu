mod cpu;
mod error;
mod memory;
mod opc;
#[cfg(test)]
mod test;
mod util;
fn main() {
    // Read pc from $FFFC and $FFFD
    use crate::cpu::Cpu;
    use crate::memory::SimpleMemory;

    let mem = util::new_mem_with_asm("ORA #$20\nORA#$45").unwrap();
    let cpu = Cpu::with_mem(mem);

    let stdin = std::io::stdin();

    let callable = |cpu: &Cpu<SimpleMemory>| {
        let mut buf = String::new();
        print!("\r{cpu}\n");
        stdin.read_line(&mut buf).unwrap();
    };

    cpu.run(&callable).unwrap();
}
