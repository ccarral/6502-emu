use crate::error::Result6052;
use crate::memory::Memory;
use crate::opc::{self, AddressMode, Inst};

pub struct Cpu<M> {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: Flags,
    sp: u8,
    mem: M,
}

pub struct Flags {
    n: bool,
    v: bool,
    b: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            n: false,
            v: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
}

impl<M> Cpu<M>
where
    M: Memory + Sized,
{
    pub fn with_mem(mem: M) -> Self {
        Self {
            pc: 0x0000,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sr: Default::default(),
            sp: 0x00,
            mem,
        }
    }
    /// Convert u16 pc to usize so it can be used to address memory
    pub fn pc(&self) -> usize {
        // Mask pc to u16::MAX
        self.pc as usize
    }

    pub(crate) fn ac(&self) -> u8 {
        self.ac
    }

    pub(crate) fn fetch_next_inst(&self) -> Inst {
        // Read byte at pc
        let byte = self.mem.read_byte(self.pc());
        opc::get_inst(byte)
    }

    pub fn execute_inst(&mut self, inst: Inst) -> Result6052 {
        // Should "panic" if the program is not well formed
        match inst {
            Inst::Adc(_) => todo!(),
            Inst::And(_) => todo!(),
            Inst::Asl(_) => todo!(),
            Inst::Bcc(_) => todo!(),
            Inst::Bcs(_) => todo!(),
            Inst::Beq(_) => todo!(),
            Inst::Bit(_) => todo!(),
            Inst::Bmi(_) => todo!(),
            Inst::Bne(_) => todo!(),
            Inst::Bpl(_) => todo!(),
            Inst::Brk(_) => todo!(),
            Inst::Bvc(_) => todo!(),
            Inst::Bvs(_) => todo!(),
            Inst::Clc(_) => todo!(),
            Inst::Cld(_) => todo!(),
            Inst::Cli(_) => todo!(),
            Inst::Clv(_) => todo!(),
            Inst::Cmp(_) => todo!(),
            Inst::Cpx(_) => todo!(),
            Inst::Cpy(_) => todo!(),
            Inst::Dec(_) => todo!(),
            Inst::Dex(_) => todo!(),
            Inst::Dey(_) => todo!(),
            Inst::Eor(_) => todo!(),
            Inst::Inc(_) => todo!(),
            Inst::Inx(_) => todo!(),
            Inst::Iny(_) => todo!(),
            Inst::Jmp(_) => todo!(),
            Inst::Jsr(_) => todo!(),
            Inst::Lda(_) => todo!(),
            Inst::Ldx(_) => todo!(),
            Inst::Ldy(_) => todo!(),
            Inst::Lsr(_) => todo!(),
            Inst::Nop(_) => todo!(),
            Inst::Ora(address_mode) => {
                let or_operand = match address_mode {
                    AddressMode::Imm => {
                        // Read one byte after pc
                        self.mem.read_byte(self.pc() + 1)
                    }
                    AddressMode::Zpg => {
                        // Read Zero Page address $00LL
                        let addr = self.mem.read_byte(self.pc() + 1);
                        self.mem.read_byte(addr as usize)
                    }
                    AddressMode::ZpgX => {
                        // Read zero page address $00LL
                        let addr = self.mem.read_byte(self.pc() + 1);
                        let effective_addr = addr + self.x;
                        effective_addr
                    }
                    AddressMode::Abs => todo!(),
                    AddressMode::AbsX => todo!(),
                    AddressMode::AbsY => todo!(),
                    AddressMode::IndX => todo!(),
                    AddressMode::IndY => todo!(),
                    _ => unreachable!(),
                };

                // or with acc
                self.ac |= or_operand;

                Ok(())
            }
            Inst::Pha(_) => todo!(),
            Inst::Php(_) => todo!(),
            Inst::Pla(_) => todo!(),
            Inst::Plp(_) => todo!(),
            Inst::Rol(_) => todo!(),
            Inst::Ror(_) => todo!(),
            Inst::Rti(_) => todo!(),
            Inst::Rts(_) => todo!(),
            Inst::Sbc(_) => todo!(),
            Inst::Sec(_) => todo!(),
            Inst::Sed(_) => todo!(),
            Inst::Sei(_) => todo!(),
            Inst::Sta(_) => todo!(),
            Inst::Stx(_) => todo!(),
            Inst::Sty(_) => todo!(),
            Inst::Tax(_) => todo!(),
            Inst::Tay(_) => todo!(),
            Inst::Tsx(_) => todo!(),
            Inst::Txa(_) => todo!(),
            Inst::Txs(_) => todo!(),
            Inst::Tya(_) => todo!(),
            Inst::None => todo!(),
        }
    }

    pub(crate) fn set_ac(&mut self, val: u8) {
        self.ac = val;
    }
}
impl<M> std::fmt::Display for Cpu<M>
where
    M: Memory,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\
Registers:
    PC {:#04x}
    AC {:#04x}",
            self.pc(),
            self.ac
        ))
    }
}
