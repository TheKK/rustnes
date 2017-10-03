use std::convert::From;

use cpu::Registers;
use cpu::Memory;

macro_rules! opcodes {
    (
        $(
            ($opcode: ident,
             $opcode_hex: expr,
             $operands_size: expr,
             $cycles_num: expr,
             $opcode_fn: expr)
        ),*
    ) => {
        pub enum OpCode {
            $(
               $opcode,
            )*
        }

        impl OpCode {
            pub fn operands_num(&self) -> u8 {
                match self {
                    $(
                        &OpCode::$opcode => $operands_size,
                    )*
                }
            }

            pub fn cycles_num(&self) -> u8 {
                match self {
                    $(
                        &OpCode::$opcode => $cycles_num,
                    )*
                }
            }

            pub fn get_fn(&self) -> fn(&mut Registers, &mut Memory) {
                match self {
                    $(
                        &OpCode::$opcode => $opcode_fn,
                    )*
                }
            }
        }

        impl From<u8> for OpCode {
            fn from(byte: u8) -> OpCode {
                match byte {
                    $(
                        $opcode_hex => OpCode::$opcode,
                    )*
                    _ => unimplemented!(),
                }
            }
        }

        impl Into<u8> for OpCode {
            fn into(self) -> u8 {
                match self {
                    $(
                        OpCode::$opcode => $opcode_hex,
                    )*
                }
            }
        }
    }
}

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

opcodes!(
    (LdaImm, 0xA9, 1, 2, lda_imm),
    (LdaZeroPage, 0xA5, 1, 3, lda_zero_page),
    (LdaZeroPageX, 0xB5, 1, 4, lda_zero_page_x),
    (LdaAbs, 0xAD, 2, 4, lda_abs),
    (Nop, 0xEA, 0, 2, nop)
);
