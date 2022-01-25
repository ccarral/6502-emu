use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error6502 {
    InvalidInstruction(usize),
}

impl Display for Error6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error6502::InvalidInstruction(pos) => f.write_fmt(format_args!(
                "Invalid instruction encountered at pos. {}",
                pos
            )),
        }
    }
}

impl Error for Error6502 {}

pub type Result6052 = Result<(), Error6502>;
