pub trait Memory {
    fn write_byte(&mut self, addr: u16, byte: u8);
    fn read_byte(&self, addr: u16) -> u8;
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
            mem.write_byte(idx as u16, *b);
        }
        mem
    }
}

impl Memory for SimpleMemory {
    fn write_byte(&mut self, addr: u16, byte: u8) {
        self.inner[addr as usize] = byte;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.inner[addr as usize]
    }
}
