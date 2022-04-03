<p align="center">
  <img src="./resources/mini6502_logo.png" width="60%"/>
</p>

# mini6502
A lightweight and simple 6502 emulation library and CLI.

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/ccarral/mini6502/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/ccarral/mini6502/actions/workflows/ci.yml 
## Implementation checklist

### Registers

Status Register (Flags)
  - [x] Negative flag (N)
  - [x] Overflow flag (V)
  - [x] Break flag (B)
  - [ ] Decimal (D)
  - [x] Interrupt (I)
  - [x] Zero flag (Z)
  - [x] Carry flag (C)

### Instructions

- [x] ADC
- [x] AND
- [x] ASL
- [x] BCC
- [x] BCS
- [x] BEQ
- [x] BIT
- [x] BMI
- [x] BNE
- [x] BPL
- [x] BRK
- [x] BVC
- [x] BVS
- [x] CLC
- [x] CLD
- [x] CLI
- [x] CLV
- [x] CMP
- [x] CPX
- [x] CPY
- [x] DEC
- [x] DEX
- [x] DEY
- [x] EOR
- [x] INC
- [x] INX
- [x] INY
- [x] JMP
- [x] JSR
- [x] LDA
- [x] LDX
- [x] LDY
- [x] LSR
- [x] NOP
- [x] ORA
- [x] PHA
- [x] PHP
- [x] PLA
- [x] PLP
- [x] ROL
- [x] ROR
- [x] RTI
- [x] RTS
- [x] SBC
- [x] SEC
- [x] SED
- [x] SEI
- [x] STA
- [x] STX
- [x] STY
- [ ] TAX
- [ ] TAY
- [ ] TSX
- [ ] TXA
- [ ] TXS
- [ ] TYA
