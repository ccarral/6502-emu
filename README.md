<p align="center">
  <img src="./resources/mini6502_logo.png"/>
</p>

# mini6502

A lightweight and simple 6502 emulation library.

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/ccarral/mini6502/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/ccarral/mini6502/actions/workflows/ci.yml 

## Example

```rust
use mini6502::{Cpu, SimpleMemory}

let rom : Vec<u8> = std::fs::read("my_rom.bin").unwrap();

let memory = SimpleMemory::from_rom(&rom);

let mut cpu = Cpu::with_mem(memory);

let stdin = std::io::stdin();

// Pass a closure that will be run every time before stepping an instruction.
cpu.run(|cpu: &Cpu<SimpleMemory>|{
    println!("{cpu}");
    //      Instruction: LDA
    //      Registers:
    //      PC 0x0600
    //      AC 0x00
    //      X: 0x00
    //      Y: 0x00
    //      SP:0x00ff
    //
    //      Flags:
    //      NV-BDIZC
    //      01110001

    // Stop until PC reaches 0x35db
    cpu.pc() == 0x35db
})
```
## Implementation checklist

- [x] Official opcodes
- [ ] "Illegal" opcodes
- [ ] Decimal mode
