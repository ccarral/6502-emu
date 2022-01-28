mod cpu;
mod error;
mod memory;
mod opc;
mod test;
mod util;
fn main() {
    use crate::cpu::Cpu;
    use crate::memory::SimpleMemory;

    let mem = util::new_mem_with_asm("ORA #$20\nORA#$45").unwrap();
    let cpu = Cpu::with_mem(mem);

    let stdin = std::io::stdin();
    let mut buf = String::new();

    let mut callable = |cpu: &Cpu<SimpleMemory>| {
        print!("\r{cpu}\n");
        stdin.read_line(&mut buf).unwrap();
    };

    cpu.run(&mut callable).unwrap();
}
