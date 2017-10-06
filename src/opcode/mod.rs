#[macro_use]
mod utils;

mod lda;
mod nop;

use std::convert::From;

use cpu::Registers;
use cpu::Memory;

use self::lda::*;
use self::nop::*;

pub struct Cycle(pub u32);

macro_rules! opcodes {
    (
        $(
            ($opcode: ident,
             $opcode_hex: expr,
             $operands_size: expr,
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

            pub fn get_fn(&self) -> fn(&mut Registers, &mut Memory) -> Cycle {
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

opcodes!(
    (LdaImm, 0xA9, 1, lda_imm),
    (LdaZeroPage, 0xA5, 1, lda_zero_page),
    (LdaZeroPageX, 0xB5, 1, lda_zero_page_x),
    (LdaAbs, 0xAD, 2, lda_abs),
    (LdaAbsX, 0xBD, 2, lda_abs_x),
    (LdaAbsY, 0xB9, 2, lda_abs_y),
    (LdaIndirectX, 0xA1, 1, lda_indirect_x),
    (LdaIndirectY, 0xB1, 1, lda_indirect_y),

    (Nop, 0xEA, 0, nop)
);
