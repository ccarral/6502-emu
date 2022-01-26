use crate::error::Error6502;
use crate::memory::Memory;
use crate::opc::{self, AddressMode, Inst, OpMode};
use crate::util;

pub struct Cpu<M> {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8,
    mem: M,
    current_inst_cycles_left: u8,
}
const N_FLAG_BITMASK: u8 = 0b10000000;
const V_FLAG_BITMASK: u8 = 0b01000000;
const B_FLAG_BITMASK: u8 = 0b00010000;
const D_FLAG_BITMASK: u8 = 0b00001000;
const I_FLAG_BITMASK: u8 = 0b00000100;
const Z_FLAG_BITMASK: u8 = 0b00000010;
const C_FLAG_BITMASK: u8 = 0b00000001;

impl<M> Cpu<M>
where
    M: Memory + Sized,
{
    pub fn with_mem(mem: M) -> Self {
        // Read PC from $FFFC and $FFFD
        let high = mem.read_byte(0xFFFC);
        let low = mem.read_byte(0xFFFD);
        let pc = util::combine_u8_to_u16(low, high);
        Self {
            pc,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sr: 0x00,
            sp: 0x00,
            mem,
            current_inst_cycles_left: 0,
        }
    }

    /// Run program loaded in cpu. A closure can be optionally passed
    /// if we want to do something (displaying registers, etc.) with the `Cpu`
    /// on each cycle.
    ///
    /// # Arguments
    ///
    /// * `callable` - Closure that takes an immutable reference to the Cpu.  
    ///
    /// # Errors
    ///
    /// Fails whenever fetching the next (valid) instruction fails.
    pub fn run(mut self, callable: &mut dyn FnMut(&Cpu<M>)) -> Result<(), Error6502> {
        while !self.exit() {
            self.step()?;
            callable(&self);
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Error6502> {
        if self.current_inst_cycles_left > 0 {
            self.current_inst_cycles_left -= 1
        } else {
            let OpMode(inst, addr_mode, cycles) = self.fetch_next_inst()?;
            self.execute_inst(inst, addr_mode)?;
            self.current_inst_cycles_left = cycles - 1;
        }
        Ok(())
    }

    pub fn exit(&self) -> bool {
        false
    }

    /// Checks if value is Zero and updates Z flag accordingly
    ///
    /// # Arguments
    ///
    /// * `val` - value to be checked
    pub fn update_z_flag_with(&mut self, val: u8) {
        if val == 0x00 {
            self.sr |= Z_FLAG_BITMASK;
        } else {
            self.sr &= !Z_FLAG_BITMASK;
        }
    }

    /// Checks if value is negative and updates N flag accordingly
    ///
    /// # Arguments
    ///
    /// * `val` - value to be checked
    pub fn update_n_flag_with(&mut self, val: u8) {
        const NEG_BITMASK: u8 = 0b10000000;
        if val & NEG_BITMASK == NEG_BITMASK {
            self.sr |= N_FLAG_BITMASK;
        } else {
            self.sr &= !N_FLAG_BITMASK;
        }
    }

    /// Convert u16 pc to usize so it can be used to address memory
    pub fn pc_usize(&self) -> usize {
        // Mask pc to u16::MAX
        self.pc as usize & 0xFFFF
    }

    pub(crate) fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub(crate) fn set_x(&mut self, val: u8) {
        self.x = val;
    }

    pub(crate) fn add_to_pc(&mut self, val: u16) {
        self.pc += val;
    }

    pub(crate) fn ac(&self) -> u8 {
        self.ac
    }

    pub fn or_flags(&mut self, mask: u8) {
        self.sr |= mask;
    }

    pub fn n_flag(&self) -> bool {
        (self.sr & N_FLAG_BITMASK) != 0
    }
    pub fn v_flag(&self) -> bool {
        (self.sr & V_FLAG_BITMASK) != 0
    }
    pub fn b_flag(&self) -> bool {
        (self.sr & B_FLAG_BITMASK) != 0
    }
    pub fn d_flag(&self) -> bool {
        (self.sr & D_FLAG_BITMASK) != 0
    }
    pub fn i_flag(&self) -> bool {
        (self.sr & I_FLAG_BITMASK) != 0
    }
    pub fn z_flag(&self) -> bool {
        (self.sr & Z_FLAG_BITMASK) != 0
    }
    pub fn c_flag(&self) -> bool {
        (self.sr & C_FLAG_BITMASK) != 0
    }

    #[inline]
    pub(crate) fn fetch_next_inst(&self) -> Result<OpMode, Error6502> {
        // Read byte at pc
        let byte = self.mem.read_byte(self.pc);
        match opc::get_op_mode(byte) {
            Some(op_mode) => Ok(op_mode),
            None => Err(Error6502::InvalidInstruction),
        }
    }

    #[inline]
    pub fn execute_inst(&mut self, inst: Inst, address_mode: AddressMode) -> Result<(), Error6502> {
        // Should "panic" if the program is not well formed
        let mut instr_len = 0u16;
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
                        instr_len = 2;
                        // Read one byte after pc
                        self.mem.read_byte(self.pc + 1)
                    }
                    AddressMode::Zpg => {
                        // Read Zero Page address $00LL
                        let addr = self.mem.read_byte(self.pc + 1);
                        instr_len = 2;
                        self.mem.read_byte(addr as u16)
                    }
                    AddressMode::ZpgX => {
                        // Read zero page address $00LL + X without carry
                        let addr = self.mem.read_byte(self.pc + 1);
                        // let effective_addr = addr + self.x;
                        let effective_addr = u8::wrapping_add(addr, self.x);
                        instr_len = 2;
                        self.mem.read_byte(effective_addr as u16)
                    }
                    AddressMode::Abs => {
                        let ll = self.mem.read_byte(self.pc + 1);
                        let hh = self.mem.read_byte(self.pc + 2);
                        let effective_addr = util::combine_u8_to_u16(hh, ll);
                        instr_len = 3;
                        self.mem.read_byte(effective_addr)
                    }
                    AddressMode::AbsX => {
                        let ll = self.mem.read_byte(self.pc + 1);
                        let hh = self.mem.read_byte(self.pc + 2);
                        let effective_addr = util::combine_u8_to_u16(hh, ll) + self.x as u16;
                        instr_len = 3;
                        self.mem.read_byte(effective_addr)
                    }
                    AddressMode::AbsY => todo!(),
                    AddressMode::IndX => todo!(),
                    AddressMode::IndY => todo!(),
                    _ => unreachable!(),
                };

                // or with acc
                self.ac |= or_operand;

                // Update N flag
                self.update_n_flag_with(self.ac);

                // Update Z flag with value from acc
                self.update_z_flag_with(self.ac);

                self.add_to_pc(instr_len);
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

    pub fn write_to_mem(&mut self, addr: u16, byte: u8) {
        self.mem.write_byte(addr as usize, byte);
    }
}
impl<M> std::fmt::Display for Cpu<M>
where
    M: Memory,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_flag(flag: bool) -> u8 {
            match flag {
                true => 1,
                false => 0,
            }
        }
        f.write_fmt(format_args!(
            "
    Registers:
    PC {:#04x}
    AC {:#04x}

    Flags:
    NV-BDIZC
    {:08b}
    ",
            self.pc_usize(),
            self.ac,
            self.sr
        ))
    }
}
