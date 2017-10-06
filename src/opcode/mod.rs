#[macro_use]
mod utils;

mod lda;
mod ldx;
mod ldy;
mod nop;

use std::convert::From;

use cpu::Registers;
use cpu::Memory;

use self::lda::*;
use self::ldx::*;
use self::ldy::*;
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

    (LdxImm, 0xA2, 1, ldx_imm),
    (LdxZeroPage, 0xA6, 1, ldx_zero_page),
    (LdxZeroPageY, 0xB6, 1, ldx_zero_page_y),
    (LdxAbs, 0xAE, 2, ldx_abs),
    (LdxAbsY, 0xBE, 2, ldx_abs_y),

    (LdyImm, 0xA0, 1, ldy_imm),
    (LdyZeroPage, 0xA4, 1, ldy_zero_page),
    (LdyZeroPageX, 0xB4, 1, ldy_zero_page_x),
    (LdyAbs, 0xAC, 2, ldy_abs),
    (LdyAbsX, 0xBC, 2, ldy_abs_x),

    (Nop, 0xEA, 0, nop)
);
