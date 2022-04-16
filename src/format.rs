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
    PC {:#04x}
    AC {:#04x}
    X: {:#02x}
    Y: {:#02x}

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
            self.p()
        ))
    }
}
