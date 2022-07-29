use crate::memory::Memory;
use crate::Cpu;
impl<M> std::fmt::Display for Cpu<M>
where
    M: Memory,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "
    Instruction: {}
    Registers:
    PC {:#06x}
    AC {:#04x}
    X: {:#04x}
    Y: {:#04x}
    SP:{:#06x} -> [{}]

    Flags:
    NV-BDIZC
    {:08b}
    ",
            match self.ir() {
                Some(inst) => format!("{inst:?}"),
                None => "NONE".to_string(),
            },
            self.pc(),
            self.ac(),
            self.x(),
            self.y(),
            self.sp(),
            if self.sp() == 0x01FF {
                "empty".to_string()
            } else {
                let sp = &self.sp();
                let bb = self.read_byte_from_mem(*sp + 1);
                format!("{:#04x}", bb)
            },
            self.p()
        ))
    }
}
