use crate::error::Error6502;
use crate::memory::Memory;
use crate::opc::{self, AddressMode, Inst, OpMode};
use crate::util;

const N_FLAG_BITMASK: u8 = 0b10000000;
const V_FLAG_BITMASK: u8 = 0b01000000;
const B_FLAG_BITMASK: u8 = 0b00010000;
const D_FLAG_BITMASK: u8 = 0b00001000;
const I_FLAG_BITMASK: u8 = 0b00000100;
const Z_FLAG_BITMASK: u8 = 0b00000010;
const C_FLAG_BITMASK: u8 = 0b00000001;
const FLAGS_DEFAULT: u8 = 0b00100000;
const STACK_ADDR_DEFAULT: u16 = 0x01FF;
const MAX_INST_CYCLES: u8 = 6;

pub struct Cpu<M> {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u16,
    mem: M,
    cycle_count: u8,
}

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
            cycle_count: 0,
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
            callable(&self);
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
        if val & NEG_BITMASK != 0 {
            self.sr |= N_FLAG_BITMASK;
        } else {
            self.sr &= !N_FLAG_BITMASK;
        }
    }

    /// Checks if the sum of the two values overflows, updates the carry flag accordingly and
    /// returns the (wrapped) sum of the values
    pub(crate) fn write_c_flag(&mut self, carry: bool) {
        // TODO: Possibly only take one argument and test with acc register?
        if carry {
            self.sr |= C_FLAG_BITMASK;
        } else {
            self.sr &= !C_FLAG_BITMASK;
        }
    }

    pub(crate) fn write_v_flag(&mut self, overflow: bool) {
        if overflow {
            self.sr |= V_FLAG_BITMASK;
        } else {
            self.sr &= !V_FLAG_BITMASK;
        }
    }

    pub fn set_d_flag(&mut self, value: bool) {
        if value {
            self.sr |= D_FLAG_BITMASK;
        } else {
            self.sr &= !D_FLAG_BITMASK;
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

    pub(crate) fn fetch_next_inst(&self) -> Result<OpMode, Error6502> {
        // Read byte at pc
        let byte = self.mem.read_byte(self.pc);
        match opc::get_op_mode(byte) {
            Some(op_mode) => Ok(op_mode),
            None => Err(Error6502::InvalidInstruction),
        }
    }

    pub fn step_inst(&mut self, inst: Inst, address_mode: AddressMode) -> Result<(), Error6502> {
        // Should "panic" if the program is not well formed
        match inst {
            Inst::ADC => {
                let operand = {
                    match address_mode {
                        AddressMode::IMM => {
                            // Read immediate byte
                            self.read_immediate_byte()
                        }
                        _ => {
                            let effective_addr = self.get_effective_address(&address_mode);
                            self.mem.read_byte(effective_addr)
                        }
                    }
                };

                let result = if self.d_flag() {
                    // Operate in bcd mode
                    0
                } else {
                    let (result, carry) = u8::overflowing_add(self.ac, operand);
                    self.write_c_flag(carry);
                    // self.write_v_flag(self.ac, operand);
                    result
                };

                self.update_z_flag_with(result);
                self.update_n_flag_with(result);
                self.ac = result;
            }
            _ => unimplemented!(),
        }

        let instr_len = get_instr_len(&address_mode);
        self.pc += instr_len;

        Ok(())
    }

    pub(crate) fn set_ac(&mut self, val: u8) {
        self.ac = val;
    }

    pub fn write_to_mem(&mut self, addr: u16, byte: u8) {
        self.mem.write_byte(addr, byte);
    }

    pub(crate) fn read_immediate_byte(&self) -> u8 {
        self.mem.read_byte(self.pc + 1)
    }

    pub(crate) fn get_effective_address(&self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            // As accumulator, immediate and implied addressing modes are 1 byte length operators,
            // implementors of opcodes must check for these modes before calling this function.
            AddressMode::ACC => unreachable!(),
            AddressMode::IMM => unreachable!(),
            AddressMode::IMPL => unreachable!(),
            AddressMode::ZPG => {
                // Zero Page address 0LL
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = util::u8_to_u16(addr);
                effective_addr
            }
            AddressMode::ZPGX => {
                // Read zero page address 0LL + X without carry
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = u8::wrapping_add(addr, self.x);
                let effective_addr = util::u8_to_u16(effective_addr);
                effective_addr
            }
            AddressMode::ZPGY => {
                // Read zero page address 0LL + Y without carry
                let addr = self.mem.read_byte(self.pc + 1);
                let effective_addr = u8::wrapping_add(addr, self.y);
                let effective_addr = util::u8_to_u16(effective_addr);
                effective_addr
            }
            AddressMode::ABS => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let effective_addr = util::combine_u8_to_u16(hh, ll);
                effective_addr
            }
            AddressMode::ABSX => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let base = util::combine_u8_to_u16(hh, ll);
                let index = util::u8_to_u16(self.x);
                let effective_addr = u16::wrapping_add(base, index);
                effective_addr
            }
            AddressMode::ABSY => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let hh_addr = u16::wrapping_add(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let base = util::combine_u8_to_u16(hh, ll);
                let index = util::u8_to_u16(self.y);
                let effective_addr = u16::wrapping_add(base, index);
                effective_addr
            }
            AddressMode::IND => {
                // NOTE: This mode doesn't cross page boundaries.
                // If first byte of address is in $xxFF then second byte is in  $xx00
                let ll_addr = util::wrapping_add_same_page(self.pc, 1);
                let hh_addr = util::wrapping_add_same_page(self.pc, 2);
                let ll = self.mem.read_byte(ll_addr);
                let hh = self.mem.read_byte(hh_addr);
                let effective_addr = util::combine_u8_to_u16(hh, ll);
                effective_addr
            }
            AddressMode::INDX => {
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
            AddressMode::INDY => {
                let ll_addr = u16::wrapping_add(self.pc, 1);
                let ll = self.mem.read_byte(ll_addr);

                let hh_addr = u16::wrapping_add(ll_addr, 1);
                let hh = self.mem.read_byte(hh_addr);

                let effective_addr = util::combine_u8_to_u16(hh, ll);
                let effective_addr = util::wrapping_add_same_page(effective_addr, self.y);
                effective_addr
            }
            AddressMode::REL => {
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
        AddressMode::ACC => 1,
        AddressMode::ABS => 3,
        AddressMode::ABSX => 3,
        AddressMode::ABSY => 3,
        AddressMode::IMM => 2,
        AddressMode::IMPL => 1,
        AddressMode::IND => 3,
        AddressMode::INDX => 2,
        AddressMode::INDY => 2,
        AddressMode::REL => 2,
        AddressMode::ZPGX => 2,
        AddressMode::ZPGY => 2,
        AddressMode::ZPG => 2,
    }
}
