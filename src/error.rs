use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error6502 {
    UnknownOpcode(u8),
}

impl Display for Error6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error6502::UnknownOpcode(v) => {
                f.write_fmt(format_args!("Opcode {v:#?} could not be decoded"))
            }
        }
    }
}

impl Error for Error6502 {}
