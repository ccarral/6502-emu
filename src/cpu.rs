use crate::bcd;
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

pub struct Cpu<M> {
    // Program counter
    pc: u16,
    // Accumulator
    ac: u8,
    x: u8,
    y: u8,
    // Flags
    p: u8,
    // Stack pointer.
    sp: u16,
    pub mem: M,
    ir: Option<Inst>,
    cycle_count: usize,
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
            p: FLAGS_DEFAULT,
            sp: STACK_ADDR_DEFAULT,
            ir: None,
            mem,
            cycle_count: 0,
        }
    }

    /// Run program loaded in cpu, with `callback_exit` called before each instruction is executed.
    /// The cpu will execute the fetch - decode - execute cycle until it encounters an instruction
    /// that can't decode and then will return an `Err`.
    /// Alternatively, it can stop execution before that by returning `true` from `callback_exit`.
    ///```no_run
    /// use mini6502::{Cpu, SimpleMemory};
    ///
    /// let buf = [0xa2, 0x10, 0xCA, 0xD0, 0xFD];
    ///
    /// let mem = SimpleMemory::from_rom(&buf);
    ///
    /// let mut cpu = Cpu::with_mem(mem);
    ///
    /// cpu.run(&mut |cpu: &Cpu<SimpleMemory>|{
    ///     println!("ir: {:?} x:{} y:{} status reg:{}",cpu.ir() , cpu.x(), cpu.y(), cpu.p());
    ///     cpu.pc() as usize >= buf.len()
    /// });
    ///
    /// assert_eq!(cpu.x(), 0x10);
    ///```
    pub fn run(&mut self, callback_exit: &mut dyn FnMut(&Cpu<M>) -> bool) -> Result<(), Error6502> {
        let opc_arr = opc::init_opc_array();
        loop {
            // Loop until we encounter an unknown opcode
            if let Ok(OpMode(instruction, address_mode, cycles)) = self.fetch_next_inst(&opc_arr) {
                self.set_ir(instruction);
                if callback_exit(&self) {
                    break;
                }
                self.step_inst(instruction, address_mode)?;
                self.add_to_cycle_count(cycles)
            } else {
                return Err(Error6502::UnknownOpcode(self.read_byte_from_mem(self.pc)));
            }
        }
        Ok(())
    }

    pub(crate) fn stack_push(&mut self, bb: u8) {
        self.mem.write_byte(self.sp, bb);
        self.sp -= 1;
    }

    pub(crate) fn stack_pop(&mut self) -> u8 {
        self.sp += 1;
        self.mem.read_byte(self.sp)
    }

    /// Checks if value is Zero and updates Z flag accordingly
    ///
    /// # Arguments
    ///
    /// * `val` - value to be checked
    pub(crate) fn update_z_flag_with(&mut self, val: u8) {
        if val == 0x00 {
            self.p |= Z_FLAG_BITMASK;
        } else {
            self.p &= !Z_FLAG_BITMASK;
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
            self.p |= N_FLAG_BITMASK;
        } else {
            self.p &= !N_FLAG_BITMASK;
        }
    }

    /// Checks if the sum of the two values overflows, updates the carry flag accordingly and
    /// returns the (wrapped) sum of the values
    pub(crate) fn write_c_flag(&mut self, carry: bool) {
        if carry {
            self.p |= C_FLAG_BITMASK;
        } else {
            self.p &= !C_FLAG_BITMASK;
        }
    }

    pub(crate) fn write_v_flag(&mut self, overflow: bool) {
        if overflow {
            self.p |= V_FLAG_BITMASK;
        } else {
            self.p &= !V_FLAG_BITMASK;
        }
    }

    pub(crate) fn write_n_flag(&mut self, neg: bool) {
        if neg {
            self.p |= N_FLAG_BITMASK;
        } else {
            self.p &= !N_FLAG_BITMASK;
        }
    }

    pub fn write_d_flag(&mut self, value: bool) {
        if value {
            self.p |= D_FLAG_BITMASK;
        } else {
            self.p &= !D_FLAG_BITMASK;
        }
    }

    pub fn write_i_flag(&mut self, value: bool) {
        if value {
            self.p |= I_FLAG_BITMASK;
        } else {
            self.p &= !I_FLAG_BITMASK;
        }
    }

    pub fn write_z_flag(&mut self, value: bool) {
        if value {
            self.p |= Z_FLAG_BITMASK;
        } else {
            self.p &= !Z_FLAG_BITMASK;
        }
    }

    pub fn write_b_flag(&mut self, value: bool) {
        if value {
            self.p |= B_FLAG_BITMASK;
        } else {
            self.p &= !B_FLAG_BITMASK;
        }
    }

    /// Convert u16 pc to usize so it can be used to address memory
    #[inline]
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

    #[inline]
    pub fn ac(&self) -> u8 {
        self.ac
    }

    #[inline]
    pub fn p(&self) -> u8 {
        self.p
    }

    #[inline]
    pub fn sp(&self) -> u16 {
        self.sp
    }

    #[inline]
    pub fn x(&self) -> u8 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> u8 {
        self.y
    }

    #[inline]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn or_flags(&mut self, mask: u8) {
        self.p |= mask;
    }

    pub fn reset_flags(&mut self) {
        self.p = FLAGS_DEFAULT;
    }

    pub fn n_flag(&self) -> bool {
        (self.p & N_FLAG_BITMASK) != 0
    }
    pub fn v_flag(&self) -> bool {
        (self.p & V_FLAG_BITMASK) != 0
    }
    pub fn b_flag(&self) -> bool {
        (self.p & B_FLAG_BITMASK) != 0
    }
    pub fn d_flag(&self) -> bool {
        (self.p & D_FLAG_BITMASK) != 0
    }
    pub fn i_flag(&self) -> bool {
        (self.p & I_FLAG_BITMASK) != 0
    }
    pub fn z_flag(&self) -> bool {
        (self.p & Z_FLAG_BITMASK) != 0
    }
    pub fn c_flag(&self) -> bool {
        (self.p & C_FLAG_BITMASK) != 0
    }

    pub(crate) fn fetch_next_inst(
        &self,
        opc_arr: &[Option<OpMode>; 0xFF],
    ) -> Result<OpMode, Error6502> {
        // NOTE: we could define opc_arr as a global const, but then we miss initialization checks
        // of opcode repetition, as we currently can't make init_opc_array() const.

        // Read byte at pc
        dbg!(self.pc);
        let byte = dbg!(self.mem.read_byte(self.pc));
        match opc_arr[byte as usize] {
            Some(op_mode) => Ok(op_mode),
            None => Err(Error6502::UnknownOpcode(byte)),
        }
    }

    fn set_ir(&mut self, inst: Inst) {
        self.ir = Some(inst);
    }

    pub fn ir(&self) -> Option<Inst> {
        self.ir
    }

    pub fn step_inst(&mut self, inst: Inst, address_mode: AddressMode) -> Result<(), Error6502> {
        // Should "panic" if the program is not well formed
        let mut add_to_pc = true;
        match inst {
            Inst::ADC => {
                let data = {
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
                    let result = bcd::bcd_add_u8(self.ac, data);
                    let carry = result > 0b1001_1001;
                    self.write_c_flag(carry);
                    result
                } else {
                    let ac = self.ac;
                    let ac_signed = {
                        let bytes = ac.to_be_bytes();
                        i8::from_be_bytes(bytes)
                    };

                    let data_signed = {
                        let bytes = data.to_be_bytes();
                        i8::from_be_bytes(bytes)
                    };

                    // dbg!(ac_signed, data_signed);

                    let (res_1, overflow_1) = ac_signed.overflowing_add(data_signed);
                    let (_res_2, overflow_2) =
                        res_1.overflowing_add(if self.c_flag() { 1 } else { 0 });
                    let (result, carry) = ac.carrying_add(data, self.c_flag());
                    self.write_c_flag(carry);
                    self.write_v_flag(overflow_1 || overflow_2);
                    result
                };

                self.update_z_flag_with(result);
                self.update_n_flag_with(result);
                self.ac = result;
            }
            Inst::AND => {
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

                self.ac &= operand;

                self.update_n_flag_with(self.ac);
                self.update_z_flag_with(self.ac);
            }
            Inst::ASL => {
                let (is_memory, operand, address) = {
                    match address_mode {
                        AddressMode::ACC => {
                            // Read accumulator
                            (false, self.ac, 0x00)
                        }
                        _ => {
                            let effective_addr = self.get_effective_address(&address_mode);
                            (true, self.mem.read_byte(effective_addr), effective_addr)
                        }
                    }
                };

                let carry = 0b1000_0000 & operand != 0;
                let result = operand << 1;

                if is_memory {
                    self.mem.write_byte(address, result);
                } else {
                    self.ac = result;
                }

                self.update_n_flag_with(result);
                self.update_z_flag_with(result);
                self.write_c_flag(carry);
            }
            Inst::BCC => {
                // Relative addressing
                if !self.c_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BCS => {
                // Relative addressing
                if self.c_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BEQ => {
                // Relative addressing
                if self.z_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BIT => {
                let operand = {
                    let addr = self.get_effective_address(&address_mode);
                    self.mem.read_byte(addr)
                };
                let equal = self.ac == operand;
                let m7 = 0b1000_0000 & operand != 0;
                let m6 = 0b0100_0000 & operand != 0;

                // Z = 0 if equal, 1 if not
                self.write_z_flag(!equal);
                self.write_v_flag(m6);
                self.write_n_flag(m7);
            }
            Inst::BMI => {
                // Relative addressing
                if self.n_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BNE => {
                if !self.z_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BPL => {
                if !self.n_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BRK => {
                // Push pc + 2 to stack
                let pc = self.pc + 2;
                let [pc_hh, pc_ll] = pc.to_be_bytes();
                self.stack_push(pc_hh);
                self.stack_push(pc_ll);
                self.write_b_flag(true);
                self.stack_push(self.p);
                let new_pc_ll = self.mem.read_byte(0xFFFE);
                let new_pc_hh = self.mem.read_byte(0xFFFF);
                let new_pc = u16::from_be_bytes([new_pc_hh, new_pc_ll]);
                self.pc = new_pc;
                add_to_pc = false;
            }
            Inst::BVC => {
                if !self.v_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::BVS => {
                if self.v_flag() {
                    let target_addr = self.get_relative_address(self.pc + 1);
                    self.pc = target_addr;
                    add_to_pc = false;
                }
            }
            Inst::CLC => {
                self.write_c_flag(false);
            }
            Inst::CLD => {
                self.write_d_flag(false);
            }
            Inst::CLI => {
                self.write_i_flag(false);
            }
            Inst::CLV => {
                self.write_v_flag(false);
            }
            Inst::CMP => {
                let data = match address_mode {
                    AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        self.mem.read_byte(addr)
                    }
                };

                let acc = self.ac;

                // A - M
                let checked_sub = acc.checked_sub(data);
                if let Some(result) = checked_sub {
                    // A >= M
                    // No overflow
                    self.update_z_flag_with(result);
                    self.write_c_flag(true);
                    self.write_n_flag(false);
                } else {
                    // A < M
                    // Overflow
                    self.write_z_flag(false);
                    self.write_n_flag(true);
                    self.write_c_flag(false);
                }
            }
            Inst::CPX => {
                // Compare to register X
                let data = {
                    match address_mode {
                        AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                        _ => {
                            let addr = self.get_effective_address(&address_mode);
                            self.mem.read_byte(addr)
                        }
                    }
                };

                let x = self.x;

                // X - M
                let checked_sub = x.checked_sub(data);
                if let Some(result) = checked_sub {
                    // X >= M
                    // No overflow
                    self.update_z_flag_with(result);
                    self.write_c_flag(true);
                    self.write_n_flag(false);
                } else {
                    // X < M
                    // Overflow
                    self.write_z_flag(false);
                    self.write_n_flag(true);
                    self.write_c_flag(false);
                }
            }
            Inst::CPY => {
                // Compare to register Y
                let data = {
                    match address_mode {
                        AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                        _ => {
                            let addr = self.get_effective_address(&address_mode);
                            self.mem.read_byte(addr)
                        }
                    }
                };

                let y = self.y;

                // Y - M
                let checked_sub = y.checked_sub(data);
                if let Some(result) = checked_sub {
                    // Y >= M
                    // No overflow
                    self.update_z_flag_with(result);
                    self.write_c_flag(true);
                    self.write_n_flag(false);
                } else {
                    // Y < M
                    // Overflow
                    self.write_z_flag(false);
                    self.write_n_flag(true);
                    self.write_c_flag(false);
                }
            }
            Inst::DEC => {
                let (addr, operand) = {
                    let addr = self.get_effective_address(&address_mode);
                    (addr, self.mem.read_byte(addr))
                };
                let result = operand.wrapping_sub(1);
                self.mem.write_byte(addr, result);
                self.update_n_flag_with(result);
                self.update_z_flag_with(result);
            }
            Inst::DEX => {
                let x = self.x;
                let x = x.wrapping_sub(1);
                self.x = x;
                self.update_z_flag_with(x);
                self.update_n_flag_with(x);
            }
            Inst::DEY => {
                let y = self.y;
                let y = y.wrapping_sub(1);
                self.y = y;
                self.update_z_flag_with(y);
                self.update_n_flag_with(y);
            }
            Inst::EOR => {
                let operand = match address_mode {
                    AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        self.mem.read_byte(addr)
                    }
                };

                let acc = self.ac;
                let result = acc ^ operand;
                self.ac = result;
                self.update_z_flag_with(result);
                self.update_n_flag_with(result);
            }
            Inst::INC => {
                let (addr, operand) = {
                    let addr = self.get_effective_address(&address_mode);
                    (addr, self.mem.read_byte(addr))
                };
                let result = operand.wrapping_add(1);
                self.mem.write_byte(addr, result);
                self.update_n_flag_with(result);
                self.update_z_flag_with(result);
            }
            Inst::INX => {
                let x = self.x;
                self.x = x.wrapping_add(1);
                self.update_n_flag_with(self.x);
                self.update_z_flag_with(self.x);
            }
            Inst::INY => {
                let y = self.y;
                self.y = y.wrapping_add(1);
                self.update_n_flag_with(self.y);
                self.update_z_flag_with(self.y);
            }
            Inst::JMP => {
                let addr = self.get_effective_address(&address_mode);
                self.pc = addr;
                add_to_pc = false;
            }
            Inst::JSR => {
                let pc = self.pc + 2;
                let [pc_ll, pc_hh] = pc.to_be_bytes();
                self.stack_push(pc_hh);
                self.stack_push(pc_ll);
                let addr = self.get_effective_address(&address_mode);
                self.pc = addr;
                add_to_pc = false;
            }
            Inst::LDA => {
                let data = match address_mode {
                    AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        self.mem.read_byte(addr)
                    }
                };
                self.set_ac(data);
                self.update_z_flag_with(data);
                self.update_n_flag_with(data);
            }
            Inst::LDX => {
                let data = match address_mode {
                    AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        self.mem.read_byte(addr)
                    }
                };
                self.set_x(data);
                self.update_z_flag_with(data);
                self.update_n_flag_with(data);
            }
            Inst::LDY => {
                let data = match address_mode {
                    AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        self.mem.read_byte(addr)
                    }
                };
                self.set_y(data);
                self.update_z_flag_with(data);
                self.update_n_flag_with(data);
            }
            Inst::LSR => {
                let (is_acc, address, operand) = match address_mode {
                    AddressMode::ACC => (true, 0x0000, self.ac),
                    _ => {
                        let addr = self.get_effective_address(&address_mode);
                        let operand = self.mem.read_byte(addr);
                        (false, addr, operand)
                    }
                };

                // Set c flag if bit 0 of ac is set
                self.write_c_flag(operand & 0b00000001 != 0);

                let result = operand >> 1;
                self.update_z_flag_with(result);

                if is_acc {
                    self.ac = result;
                } else {
                    self.write_to_mem(address, result);
                }
            }
            Inst::NOP => {
                // TODO: enable asm features
                // asm!("NOP");
            }
            Inst::ORA => {
                let data = {
                    match address_mode {
                        AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                        _ => {
                            let addr = self.get_effective_address(&address_mode);
                            self.mem.read_byte(addr)
                        }
                    }
                };

                let result = self.ac | data;

                self.update_n_flag_with(result);
                self.update_z_flag_with(result);

                self.ac = result;
            }
            Inst::PHA => {
                self.stack_push(self.ac);
            }
            Inst::PHP => {
                self.stack_push(self.p);
            }
            Inst::PLA => {
                let ac = self.stack_pop();
                self.ac = ac;
                self.update_z_flag_with(ac);
                self.update_n_flag_with(ac);
            }
            Inst::PLP => {
                let p = self.stack_pop();
                self.p = p;
            }
            Inst::ROL => {
                let (is_memory, operand, address) = {
                    match address_mode {
                        AddressMode::ACC => {
                            // Read accumulator
                            (false, self.ac, 0x00)
                        }
                        _ => {
                            let effective_addr = self.get_effective_address(&address_mode);
                            (true, self.mem.read_byte(effective_addr), effective_addr)
                        }
                    }
                };

                let carry_out = 0b1000_0000 & operand != 0;
                let mut result = operand << 1;

                if self.c_flag() {
                    result |= 0b0000_0001;
                }

                if is_memory {
                    self.mem.write_byte(address, result);
                } else {
                    self.ac = result;
                }

                self.update_n_flag_with(result);
                self.update_z_flag_with(result);
                self.write_c_flag(carry_out);
            }
            Inst::ROR => {
                let (is_memory, operand, address) = {
                    match address_mode {
                        AddressMode::ACC => {
                            // Read accumulator
                            (false, self.ac, 0x00)
                        }
                        _ => {
                            let effective_addr = self.get_effective_address(&address_mode);
                            (true, self.mem.read_byte(effective_addr), effective_addr)
                        }
                    }
                };

                let carry_out = 0b0000_0001 & operand != 0;
                let mut result = operand >> 1;

                if self.c_flag() {
                    result |= 0b1000_0000;
                }

                if is_memory {
                    self.mem.write_byte(address, result);
                } else {
                    self.ac = result;
                }

                self.update_n_flag_with(result);
                self.update_z_flag_with(result);
                self.write_c_flag(carry_out);
            }
            Inst::RTI => {
                let p = self.stack_pop();
                let pc_ll = self.stack_pop();
                let pc_hh = self.stack_pop();
                let pc = u16::from_be_bytes([pc_hh, pc_ll]);
                self.p = p;
                self.pc = pc;
                add_to_pc = false;
            }
            Inst::RTS => {
                let pc_ll = self.stack_pop();
                let pc_hh = self.stack_pop();
                let pc = u16::from_be_bytes([pc_ll, pc_hh]);
                self.pc = pc + 1;
                add_to_pc = false;
            }
            Inst::SBC => {
                let data = {
                    match address_mode {
                        AddressMode::IMM => self.mem.read_byte(self.pc + 1),
                        _ => {
                            let addr = self.get_effective_address(&address_mode);
                            self.mem.read_byte(addr)
                        }
                    }
                };

                if self.d_flag() {
                    // TODO: implement bcd mode
                    unimplemented!();
                } else {
                    let ac_signed = {
                        let ac_bytes = self.ac.to_be_bytes();
                        i8::from_be_bytes(ac_bytes)
                    };

                    let data_signed = {
                        let data_bytes = data.to_be_bytes();
                        i8::from_be_bytes(data_bytes)
                    };

                    let data_signed_neg_u8 = {
                        let data_signed_neg = -data_signed;
                        let data_signed_neg_bytes = data_signed_neg.to_be_bytes();
                        u8::from_be_bytes(data_signed_neg_bytes)
                    };

                    let (result_1, overflow_1) = ac_signed.overflowing_sub(data_signed);

                    let (_, carry_1) = self.ac.overflowing_add(data_signed_neg_u8);

                    let result_1_u8 = {
                        let bytes = result_1.to_be_bytes();
                        u8::from_be_bytes(bytes)
                    };

                    let (result_2, overflow_2) =
                        result_1.overflowing_sub(if self.c_flag() { 0 } else { 1 });

                    let (_result_2_u8, carry_2) =
                        result_1_u8.overflowing_add(if self.c_flag() { 0 } else { 1 });

                    let result_bytes = result_2.to_be_bytes();

                    self.ac = u8::from_be_bytes(result_bytes);

                    self.update_n_flag_with(self.ac);
                    self.write_v_flag(overflow_1 || overflow_2);
                    self.update_z_flag_with(self.ac);
                    self.write_c_flag(carry_1 || carry_2);
                }
            }
            Inst::SEC => {
                self.write_c_flag(true);
            }
            Inst::SED => {
                self.write_d_flag(true);
            }
            Inst::SEI => {
                self.write_i_flag(true);
            }
            Inst::STA => {
                let address = self.get_effective_address(&address_mode);
                let ac = self.ac;
                self.write_to_mem(address, ac);
            }
            Inst::STX => {
                let address = self.get_effective_address(&address_mode);
                let x = self.x;
                self.write_to_mem(address, x);
            }
            Inst::STY => {
                let address = self.get_effective_address(&address_mode);
                let y = self.y;
                self.write_to_mem(address, y);
            }
            Inst::TAX => {
                self.x = self.ac;
                self.update_n_flag_with(self.ac);
                self.update_z_flag_with(self.ac);
            }
            Inst::TAY => {
                self.y = self.ac;
                self.update_n_flag_with(self.ac);
                self.update_z_flag_with(self.ac);
            }
            Inst::TSX => {
                let [_sp_hh, sp_ll] = self.sp.to_be_bytes();
                self.x = sp_ll;
                self.update_z_flag_with(self.x);
                self.update_n_flag_with(self.x);
            }
            Inst::TXA => {
                self.ac = self.x;
                self.update_n_flag_with(self.ac);
                self.update_z_flag_with(self.ac);
            }
            Inst::TXS => {
                self.sp = u16::from_be_bytes([0x01, self.x]);
            }
            Inst::TYA => {
                self.ac = self.y;
                self.update_n_flag_with(self.ac);
                self.update_z_flag_with(self.ac);
            }
        }

        if add_to_pc {
            let instr_len = get_instr_len(&address_mode);
            self.pc += instr_len;
        }

        Ok(())
    }

    pub(crate) fn set_ac(&mut self, val: u8) {
        self.ac = val;
    }

    pub(crate) fn write_to_mem(&mut self, addr: u16, byte: u8) {
        self.mem.write_byte(addr, byte);
    }
    pub(crate) fn read_byte_from_mem(&self, addr: u16) -> u8 {
        self.mem.read_byte(addr)
    }

    pub(crate) fn read_immediate_byte(&self) -> u8 {
        self.mem.read_byte(self.pc + 1)
    }

    pub(crate) fn add_to_cycle_count(&mut self, cycles: u8) {
        self.cycle_count += cycles as usize;
    }

    /// Get relative address for jump instruction, min -128 and max 127
    pub(crate) fn get_relative_address(&self, offset_address: u16) -> u16 {
        let offset = self.mem.read_byte(offset_address);
        let offset_16 = {
            if util::test_negative(offset) {
                // Number is negative, extend with 0xFF
                util::combine_u8_to_u16(0xFF, offset)
            } else {
                util::combine_u8_to_u16(0x00, offset)
            }
        };
        // Explanation:
        // The jump is calculated from the offset address + 1, because in the real 6502, you
        // consume one byte at a time and increment the program counter accordingly. However, this
        // emulator doesn't increment the pc until after the whole instruction is executed.
        offset_16.wrapping_add(offset_address).wrapping_add(1)
    }

    pub(crate) fn get_effective_address(&self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            // As accumulator, immediate and implied addressing modes are 1 byte length operators,
            // implementors of opcodes must check for these modes before calling this function.
            AddressMode::ACC => unreachable!(),
            AddressMode::IMM => unreachable!(),
            AddressMode::IMPL => unreachable!(),
            // Relative addressing was moved to it's own function, as the couple instructions
            // that use, they use it exclusively, so it saves a lookup
            AddressMode::REL => unreachable!(),
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
        }
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
