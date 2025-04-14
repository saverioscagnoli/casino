use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Hello = 0,
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::Hello => 0,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Hello),
            _ => Err(()),
        }
    }
}
