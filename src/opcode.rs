use std::convert::From;

use cpu::Registers;
use cpu::Memory;

pub fn nop(_registers: &mut Registers, _mem: &mut Memory) {}
pub fn lda_imm(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let val = mem.read((pc + 1) as u16);

    registers.a = val;
}
pub fn lda_zero_page(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let addr = mem.read((pc + 1) as u16);

    registers.a = mem.read(addr as u16);
}

pub enum OpCode {
    Nop,
    LdaImm,
    LdaZeroPage,
}

impl OpCode {
    pub fn operands_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 1,
            &OpCode::LdaZeroPage => 1,
            &OpCode::Nop => 0,
            _ => unimplemented!(),
        }
    }

    pub fn cycles_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 2,
            &OpCode::LdaZeroPage => 3,
            &OpCode::Nop => 2,
            _ => unimplemented!(),
        }
    }

    pub fn get_fn(&self) -> fn(&mut Registers, &mut Memory) {
        match self {
            &OpCode::LdaImm => lda_imm,
            &OpCode::LdaZeroPage => lda_zero_page,
            &OpCode::Nop => nop,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0xA9 => OpCode::LdaImm,
            0xA5 => OpCode::LdaZeroPage,
            0xEA => OpCode::Nop,
            _ => unimplemented!(),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::LdaImm => 0xA9,
            OpCode::LdaZeroPage => 0xA5,
            OpCode::Nop => 0xEA,
            _ => unimplemented!(),
        }
    }
}
