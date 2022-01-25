pub trait Memory {
    fn set_byte(&mut self, addr: u16, byte: u8);
    fn read_byte(&self, addr: u16) -> u8;
}

struct SimpleMemory {
    inner: [u8; 0x10000],
}

impl SimpleMemory {
    pub fn new() -> Self {
        SimpleMemory {
            inner: [0; 0x10000],
        }
    }

    pub fn from_rom(rom: &[u8]) -> SimpleMemory {
        let mut mem = SimpleMemory::new();
        for (idx, b) in rom.into_iter().enumerate() {
            mem.set_byte(idx as u16, *b);
        }
        mem
    }
}

impl Memory for SimpleMemory {
    fn set_byte(&mut self, addr: u16, byte: u8) {
        self.inner[addr as usize] = byte;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.inner[addr as usize]
    }
}
