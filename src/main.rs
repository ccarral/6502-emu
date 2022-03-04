pub fn main() {
    use clap::{Arg, ArgGroup, Command};
    use mini6502::cpu::Cpu;
    use mini6502::memory::SimpleMemory;

    let matches = Command::new("mini6502")
        .author("Carlos Carral")
        .arg(Arg::new("asm").long("asm").short('a').value_name("FILE"))
        .arg(Arg::new("bin").long("bin").short('b').value_name("FILE"))
        .group(
            ArgGroup::new("input")
                .args(&["asm", "bin"])
                .required(true)
                .multiple(false),
        )
        .get_matches();

    let (input_file, is_asm) = {
        if let Some(asm_file) = matches.value_of("asm") {
            (asm_file, true)
        } else if let Some(bin_file) = matches.value_of("bin") {
            (bin_file, false)
        } else {
            unreachable!();
        }
    };

    let mem = mini6502::util::new_mem_with_asm("ORA #$20\nORA#$45").unwrap();
    let cpu = Cpu::with_mem(mem);

    let stdin = std::io::stdin();
    let mut buf = String::new();

    let mut callable = |cpu: &Cpu<SimpleMemory>| {
        print!("\r{cpu}\n");
        stdin.read_line(&mut buf).unwrap();
    };

    // cpu.run(&mut callable).unwrap();
}
