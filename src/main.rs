use clap::{Arg, Command};
use mini6502::cpu::Cpu;
use mini6502::memory::SimpleMemory;
use std::error::Error;
use std::fs;
pub fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("mini6502")
        .author("Carlos Carral")
        .arg(Arg::new("bin").value_name("FILE"))
        .arg(
            Arg::new("step")
                .long("--step")
                .short('s')
                .required(false)
                .help("Step trough every instruction."),
        )
        .get_matches();

    let file_name = matches.value_of("bin").unwrap();

    match fs::read(file_name) {
        Ok(contents) => {
            let mem = SimpleMemory::from_rom(&contents);
            let cpu = Cpu::with_mem(mem);
            let stdin = std::io::stdin();
            let mut buf = String::new();

            let mut callable = |cpu: &Cpu<SimpleMemory>| {
                print!("\r{cpu}\n");
                stdin.read_line(&mut buf).unwrap();
            };
            cpu.run(&mut callable)?;
        }
        Err(os_err_msg) => {
            eprintln!("Error while opening file \"{file_name}\": {os_err_msg}");
            std::process::exit(-1);
        }
    };

    Ok(())
}
