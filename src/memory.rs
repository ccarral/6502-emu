pub trait Memory {
    fn set_byte(&mut self, addr: usize, byte: u8);
    fn read_byte(&self, addr: usize) -> u8;
}

pub struct SimpleMemory {
    inner: [u8; 0x10000],
}

impl SimpleMemory {
    fn new() -> Self {
        SimpleMemory {
            inner: [0; 0x10000],
        }
    }

    pub fn from_rom(rom: &[u8]) -> SimpleMemory {
        let mut mem = SimpleMemory::new();
        for (idx, b) in rom.into_iter().enumerate() {
            mem.set_byte(idx, *b);
        }
        mem
    }
}

impl Memory for SimpleMemory {
    fn set_byte(&mut self, addr: usize, byte: u8) {
        self.inner[addr as usize] = byte;
    }

    fn read_byte(&self, addr: usize) -> u8 {
        self.inner[addr]
    }
}
