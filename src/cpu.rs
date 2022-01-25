use crate::error::Error6502;
use crate::memory::Memory;
use crate::opc::{self, AddressMode, Inst, OpMode};

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

    #[inline]
    pub(crate) fn fetch_next_inst(&self) -> Result<OpMode, Error6502> {
        // Read byte at pc
        let byte = self.mem.read_byte(self.pc());
        match opc::get_op_mode(byte) {
            Some(op_mode) => Ok(op_mode),
            None => Err(Error6502::InvalidInstruction),
        }
    }

    #[inline]
    pub fn execute_inst(&mut self, inst: Inst, address_mode: AddressMode) -> Result<(), Error6502> {
        // Should "panic" if the program is not well formed
        match inst {
            Inst::Adc => todo!(),
            Inst::And => todo!(),
            Inst::Asl => todo!(),
            Inst::Bcc => todo!(),
            Inst::Bcs => todo!(),
            Inst::Beq => todo!(),
            Inst::Bit => todo!(),
            Inst::Bmi => todo!(),
            Inst::Bne => todo!(),
            Inst::Bpl => todo!(),
            Inst::Brk => todo!(),
            Inst::Bvc => todo!(),
            Inst::Bvs => todo!(),
            Inst::Clc => todo!(),
            Inst::Cld => todo!(),
            Inst::Cli => todo!(),
            Inst::Clv => todo!(),
            Inst::Cmp => todo!(),
            Inst::Cpx => todo!(),
            Inst::Cpy => todo!(),
            Inst::Dec => todo!(),
            Inst::Dex => todo!(),
            Inst::Dey => todo!(),
            Inst::Eor => todo!(),
            Inst::Inc => todo!(),
            Inst::Inx => todo!(),
            Inst::Iny => todo!(),
            Inst::Jmp => todo!(),
            Inst::Jsr => todo!(),
            Inst::Lda => todo!(),
            Inst::Ldx => todo!(),
            Inst::Ldy => todo!(),
            Inst::Lsr => todo!(),
            Inst::Nop => todo!(),
            Inst::Ora => {
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
            Inst::Pha => todo!(),
            Inst::Php => todo!(),
            Inst::Pla => todo!(),
            Inst::Plp => todo!(),
            Inst::Rol => todo!(),
            Inst::Ror => todo!(),
            Inst::Rti => todo!(),
            Inst::Rts => todo!(),
            Inst::Sbc => todo!(),
            Inst::Sec => todo!(),
            Inst::Sed => todo!(),
            Inst::Sei => todo!(),
            Inst::Sta => todo!(),
            Inst::Stx => todo!(),
            Inst::Sty => todo!(),
            Inst::Tax => todo!(),
            Inst::Tay => todo!(),
            Inst::Tsx => todo!(),
            Inst::Txa => todo!(),
            Inst::Txs => todo!(),
            Inst::Tya => todo!(),
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
