use std::convert::From;

use cpu::Registers;
use cpu::Memory;

pub fn nop(_registers: &mut Registers, _mem: &mut Memory) {}
pub fn lda_imm(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let val = mem.read((pc + 1) as u16);

    registers.a = val;
}

pub enum OpCode {
    Nop,
    LdaImm,
}

impl OpCode {
    pub fn operands_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 1,
            &OpCode::Nop => 0,
            _ => unimplemented!(),
        }
    }

    pub fn cycles_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 2,
            &OpCode::Nop => 2,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0xA9 => OpCode::LdaImm,
            0xEA => OpCode::Nop,
            _ => unimplemented!(),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::LdaImm => 0xA9,
            OpCode::Nop => 0xEA,
            _ => unimplemented!(),
        }
    }
}
