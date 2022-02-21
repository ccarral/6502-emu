use crate::cpu::Cpu;
use crate::memory::SimpleMemory;
use asm6502::assemble;

/// Combine bytes 0xHH and 0xLL into 0xHHLL
#[inline]
pub const fn combine_u8_to_u16(hh: u8, ll: u8) -> u16 {
    u16::from_be_bytes([hh, ll])
}

/// Pad a single byte 0xBB into an u16 0x00BB
#[inline]
pub const fn u8_to_u16(val: u8) -> u16 {
    val as u16
}

/// Wraps adding a displacement to an address to the same page.
///
/// # Arguments
///
/// * `base` - Base address
/// * `displ` - Displacement
///
/// # Examples
///
/// ```rust
/// use mini6502::util::wrapping_add_same_page;
/// let addr = 0x11FF;
/// let displ = 1;
/// let new_addr = wrapping_add_same_page(addr, displ);
/// assert_eq!(new_addr, 0x1100);
/// ```
#[inline]
pub const fn wrapping_add_same_page(addr: u16, displacement: u8) -> u16 {
    let [page, addr_in_page] = u16::to_be_bytes(addr);
    let wrapped_addr_in_page = u8::wrapping_add(addr_in_page, displacement);
    let new_addr = u16::from_be_bytes([page, wrapped_addr_in_page]);
    new_addr
}

#[inline]
pub const fn test_negative(val: u8) -> bool {
    const NEG_BITMASK: u8 = 0b10000000;
    val & NEG_BITMASK != 0
}

#[inline]
pub const fn test_overflow(b1: u8, b2: u8) -> bool {
    const BIT_7_MASK: u8 = 0b10000000;
    // Check if there is carry from bit 6 into bit 7 by turning off bit 7 on both operands and
    // adding them
    let bit_7_carry_in = ((b1 & !BIT_7_MASK) + (b2 & !BIT_7_MASK)) & BIT_7_MASK != 0;

    // Check if theres a carry out from bit 7
    let (_res, bit_7_carry_out) = u8::overflowing_add(b1, b2);

    // xor carry in and carry out from bit 7
    bit_7_carry_in ^ bit_7_carry_out
}

pub fn new_mem_with_asm(asm: &str) -> Result<SimpleMemory, String> {
    let mut bin = Vec::new();
    assemble(asm.as_bytes(), &mut bin)?;
    Ok(SimpleMemory::from_rom(&bin))
}

pub fn new_cpu_with_asm(asm: &str) -> Result<Cpu<SimpleMemory>, String> {
    let mem = new_mem_with_asm(asm)?;
    let cpu = Cpu::with_mem(mem);
    Ok(cpu)
}

pub fn new_cpu_empty_mem() -> Cpu<SimpleMemory> {
    let mem = SimpleMemory::from_rom(&[]);
    let cpu = Cpu::with_mem(mem);
    cpu
}

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_wrapping_add_same_page() {
        let addr = 0x12FF;
        let displ = 4;
        let new_addr = super::wrapping_add_same_page(addr, displ);
        assert_eq!(new_addr, 0x1203);
    }

    #[test]
    fn test_combine_u8_into_u16() {
        let hh = 0x90;
        let ll = 0x45;
        let hhll = super::combine_u8_to_u16(hh, ll);
        assert_eq!(hhll, 0x9045);
    }

    #[test]
    fn test_overflow() {
        // (+64) + (+65) = -127 in two's complement logic, which is an overflow error
        assert!(util::test_overflow(0b01000000, 0b01000001));
        // (-1) + (-1) = -2 which is correct, so no flag
        assert!(!util::test_overflow(0b11111111, 0b11111111));
        // (-64) + (-65) = +127 in two's complement logic, which is an overflow error
        assert!(util::test_overflow(0b11000000, 0b10111111));
    }
}
