fn main() {
    use mini6502::cpu::Cpu;
    use mini6502::memory::SimpleMemory;

    let mem = mini6502::util::new_mem_with_asm("ORA #$20\nORA#$45").unwrap();
    let cpu = Cpu::with_mem(mem);

    let stdin = std::io::stdin();
    let mut buf = String::new();

    let mut callable = |cpu: &Cpu<SimpleMemory>| {
        print!("\r{cpu}\n");
        stdin.read_line(&mut buf).unwrap();
    };

    cpu.run(&mut callable).unwrap();
}
