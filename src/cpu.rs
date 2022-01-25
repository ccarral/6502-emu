use crate::memory::Memory;

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

impl<M> Cpu<M>
where
    M: Memory,
{
    pub fn fetch_next_inst(&self) {}
}
