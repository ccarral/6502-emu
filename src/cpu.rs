use crate::error::Error6502;
use crate::memory::Memory;
use crate::opc::{self, AddressMode, Inst, OpMode, Operand};
use crate::util;

pub struct Cpu<M> {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u16,
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
const FLAGS_DEFAULT: u8 = 0b00100000;
const STACK_ADDR_DEFAULT: u16 = 0x01FF;

impl<M> Cpu<M>
where
    M: Memory + Sized,
{
    /// Create a new CPU with memory `mem`.
    ///
    /// # Arguments
    ///
    /// * `mem` - Impl. of `Memory`
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
            sr: FLAGS_DEFAULT,
            sp: STACK_ADDR_DEFAULT,
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

    pub(crate) fn stack_push(&mut self, bb: u8) {
        self.mem.write_byte(self.sp, bb);
        dbg!(self.sp);
        self.sp -= 1;
    }

    pub(crate) fn stack_pop(&mut self) -> u8 {
        self.sp += 1;
        dbg!(self.sp);
        self.mem.read_byte(self.sp)
    }

    /// Checks if value is Zero and updates Z flag accordingly
    ///
    /// # Arguments
    ///
    /// * `val` - value to be checked
    pub(crate) fn update_z_flag_with(&mut self, val: u8) {
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
    pub(crate) fn update_n_flag_with(&mut self, val: u8) {
        const NEG_BITMASK: u8 = 0b10000000;
        if val & NEG_BITMASK == NEG_BITMASK {
            self.sr |= N_FLAG_BITMASK;
        } else {
            self.sr &= !N_FLAG_BITMASK;
        }
    }

    /// Checks if the sum of the two values overflows, updates the carry flag accordingly and
    /// returns the (wrapped) sum of the values
    pub(crate) fn update_c_flag_with(&mut self, b1: u8, b2: u8) -> u8 {
        // TODO: Possibly only take one argument and test with acc register?
        let (result, carry) = dbg!(u8::overflowing_add(b1, b2));
        if carry {
            self.sr |= C_FLAG_BITMASK;
        } else {
            self.sr &= !C_FLAG_BITMASK;
        }
        result
    }

    pub(crate) fn update_v_flag_with(&mut self, b1: u8, b2: u8) -> u8 {
        const BIT_7_MASK: u8 = 0b10000000;
        // Check if there is carry from bit 6 into bit 7 by turning off bit 7 on both operands and
        // adding them
        let bit_7_carry_in = ((b1 & !BIT_7_MASK) + (b2 & !BIT_7_MASK)) & BIT_7_MASK != 0;

        // Check if theres a carry out from bit 7
        let (res, bit_7_carry_out) = u8::overflowing_add(b1, b2);

        // xor carry in and carry out from bit 7
        let set_flag = bit_7_carry_in ^ bit_7_carry_out;

        if set_flag {
            self.sr |= V_FLAG_BITMASK;
        } else {
            self.sr &= !V_FLAG_BITMASK;
        }
        res
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

    pub(crate) fn set_y(&mut self, val: u8) {
        self.y = val;
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

    pub fn reset_flags(&mut self) {
        self.sr = FLAGS_DEFAULT;
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
                let or_operand: u8 = {
                    let effective_addr = self.get_effective_address(&address_mode);
                    self.mem.read_byte(effective_addr)
                };

                let instr_len = get_instr_len(&address_mode);

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
        }
    }

    pub(crate) fn set_ac(&mut self, val: u8) {
        self.ac = val;
    }

    pub fn write_to_mem(&mut self, addr: u16, byte: u8) {
        self.mem.write_byte(addr, byte);
    }

    pub(crate) fn get_effective_address(&self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            // As accumulator, immediate and implied addressing modes are 1 byte length operators,
            // implementors of opcodes must check for these modes before calling this function.
            AddressMode::Acc => unreachable!(),
            AddressMode::Imm => unreachable!(),
            AddressMode::Impl => unreachable!(),
            AddressMode::Zpg => {
                // Zero Page address 0LL
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = util::u8_to_u16(addr);
                effective_addr
            }
            AddressMode::ZpgX => {
                // Read zero page address 0LL + X without carry
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = u8::wrapping_add(addr, self.x);
                let effective_addr = util::u8_to_u16(effective_addr);
                effective_addr
            }
            AddressMode::ZpgY => {
                // Read zero page address 0LL + Y without carry
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = u8::wrapping_add(addr, self.y);
                let effective_addr = util::u8_to_u16(effective_addr);
                effective_addr
            }
            AddressMode::Abs => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let effective_addr = util::combine_u8_to_u16(hh, ll);
                effective_addr
            }
            AddressMode::AbsX => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let base = util::combine_u8_to_u16(hh, ll);
                let index = util::u8_to_u16(self.x);
                let effective_addr = u16::wrapping_add(base, index);
                effective_addr
            }
            AddressMode::AbsY => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let base = util::combine_u8_to_u16(hh, ll);
                let index = util::u8_to_u16(self.y);
                let effective_addr = u16::wrapping_add(base, index);
                effective_addr
            }
            AddressMode::Ind => {
                // NOTE: This mode doesn't cross page boundaries.
                // If first byte of address is in $xxFF then second byte is in  $xx00
                let ll_addr = util::wrapping_add_same_page(self.pc, 1);
                let hh_addr = util::wrapping_add_same_page(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let effective_addr = util::combine_u8_to_u16(hh, ll);
                effective_addr
            }
            AddressMode::IndX => {
                let bb_addr = u16::wrapping_add(self.pc, 1);
                let bb = self.mem.read_byte(bb_addr);
                // 00BB + X no carry, no page boundary crossing
                let ind_addr_ll = u8::wrapping_add(bb, self.x);
                let ind_addr_ll_zpg = util::u8_to_u16(ind_addr_ll);
                // 00BB + X + 1 no carry, no page boundary crossing
                let ind_addr_hh = u8::wrapping_add(ind_addr_ll, 1);
                let ind_addr_hh_zpg = util::u8_to_u16(ind_addr_hh);

                let ll = self.mem.read_byte(ind_addr_ll_zpg);
                let hh = self.mem.read_byte(ind_addr_hh_zpg);

                let effective_addr = util::combine_u8_to_u16(hh, ll);
                effective_addr
            }
            AddressMode::IndY => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let ll = self.mem.read_byte(ll_addr);

                let hh_addr = u16::wrapping_add(ll_addr, 1);
                let hh = self.mem.read_byte(hh_addr);

                let effective_addr = util::combine_u8_to_u16(hh, ll);
                let effective_addr = util::wrapping_add_same_page(effective_addr, self.y);
                effective_addr
            }
            AddressMode::Rel => {
                // Get 8 bit 2's complement encoded signed offset
                let bb_addr = u16::wrapping_add(self.pc, 1);
                let offset = self.mem.read_byte(bb_addr);
                let offset_16 = {
                    if offset & 0b10000000 != 0 {
                        // Number is negative, extend with 0xFF
                        util::combine_u8_to_u16(0xFF, offset)
                    } else {
                        util::combine_u8_to_u16(0x00, offset)
                    }
                };
                let target_addr = u16::wrapping_add(offset_16, self.pc);
                target_addr
            }
        }
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

const fn get_instr_len(addr_mode: &AddressMode) -> u16 {
    match addr_mode {
        AddressMode::Acc => 1,
        AddressMode::Abs => 3,
        AddressMode::AbsX => 3,
        AddressMode::AbsY => 3,
        AddressMode::Imm => 2,
        AddressMode::Impl => 1,
        AddressMode::Ind => 3,
        AddressMode::IndX => 2,
        AddressMode::IndY => 2,
        AddressMode::Rel => 2,
        AddressMode::Zpg => 2,
        AddressMode::ZpgX => 2,
        AddressMode::ZpgY => 2,
    }
}
