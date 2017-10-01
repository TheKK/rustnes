use std::convert::From;

pub enum OpCode {
    Nop,
}

impl OpCode {
    pub fn operands_num(&self) -> u8 {
        match self {
            &OpCode::Nop => 0,
            _ => unimplemented!(),
        }
    }

    pub fn cycles_num(&self) -> u8 {
        match self {
            &OpCode::Nop => 2,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0xEA => OpCode::Nop,
            _ => unimplemented!(),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::Nop => 0xEA,
            _ => unimplemented!(),
        }
    }
}
