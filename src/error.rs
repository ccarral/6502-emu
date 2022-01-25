use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error6502 {
    InvalidInstruction,
}

impl Display for Error6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error6502::InvalidInstruction => {
                f.write_fmt(format_args!("Invalid instruction encountered"))
            }
        }
    }
}

impl Error for Error6502 {}
