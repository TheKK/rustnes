use std::convert::From;

use cpu::Registers;
use cpu::Memory;

fn nop(_registers: &mut Registers, _mem: &mut Memory) {}

fn lda_imm(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let val = mem.read((pc + 1) as u16);

    registers.a = val;
}

fn lda_zero_page(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let addr = mem.read((pc + 1) as u16);

    registers.a = mem.read(addr as u16);
}

fn lda_zero_page_x(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let x = &registers.x;
    let addr = mem.read((pc + 1) as u16) + x;

    registers.a = mem.read(addr as u16);
}

fn lda_abs(registers: &mut Registers, mem: &mut Memory) {
    let pc = &registers.pc;
    let addr_low = mem.read((pc + 1) as u16) as u16;
    let addr_high = mem.read((pc + 2) as u16) as u16;
    let addr = (addr_high << 8) + addr_low;

    registers.a = mem.read(addr);
}

pub enum OpCode {
    Nop,
    LdaImm,
    LdaZeroPage,
    LdaZeroPageX,
    LdaAbs,
}

impl OpCode {
    pub fn operands_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 1,
            &OpCode::LdaZeroPage => 1,
            &OpCode::LdaZeroPageX => 1,
            &OpCode::LdaAbs => 2,
            &OpCode::Nop => 0,
        }
    }

    pub fn cycles_num(&self) -> u8 {
        match self {
            &OpCode::LdaImm => 2,
            &OpCode::LdaZeroPage => 3,
            &OpCode::LdaZeroPageX => 4,
            &OpCode::LdaAbs => 4,
            &OpCode::Nop => 2,
        }
    }

    pub fn get_fn(&self) -> fn(&mut Registers, &mut Memory) {
        match self {
            &OpCode::LdaImm => lda_imm,
            &OpCode::LdaZeroPage => lda_zero_page,
            &OpCode::LdaZeroPageX => lda_zero_page_x,
            &OpCode::LdaAbs => lda_abs,
            &OpCode::Nop => nop,
        }
    }
}

macro_rules! impl_from_and_into {
    ($($opcode: path => $byte: expr), *) => (
        impl From<u8> for OpCode {
            fn from(byte: u8) -> OpCode {
                match byte {
                    $(
                        $byte => $opcode,
                    )*
                    _ => unimplemented!(),
                }
            }
        }

        impl Into<u8> for OpCode {
            fn into(self) -> u8 {
                match self {
                    $(
                        $opcode => $byte,
                    )*
                }
            }
        }
    )
}

impl_from_and_into! {
    OpCode::LdaImm => 0xA9,
    OpCode::LdaZeroPage => 0xA5,
    OpCode::LdaZeroPageX => 0xB5,
    OpCode::LdaAbs => 0xAD,
    OpCode::Nop => 0xEA
}
