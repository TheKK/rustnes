#[macro_use]
mod utils;

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod clc;
mod cld;
mod cli;
mod clv;
mod lda;
mod ldx;
mod ldy;
mod nop;
mod sec;
mod sed;
mod sei;

use std::convert::From;

use cpu::Registers;
use cpu::Memory;

use self::adc::*;
use self::and::*;
use self::asl::*;
use self::bcc::*;
use self::bcs::*;
use self::beq::*;
use self::bit::*;
use self::bmi::*;
use self::bne::*;
use self::bpl::*;
use self::clc::*;
use self::cld::*;
use self::cli::*;
use self::clv::*;
use self::lda::*;
use self::ldx::*;
use self::ldy::*;
use self::nop::*;
use self::sec::*;
use self::sed::*;
use self::sei::*;

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
                    _ => unreachable!(),
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
    (AdcImm, 0x69, 1, adc_imm),
    (AdcZeroPage, 0x65, 1, adc_zero_page),
    (AdcZeroPageX, 0x75, 1, adc_zero_page_x),
    (AdcAbs, 0x6D, 2, adc_abs),
    (AdcAbsX, 0x7D, 2, adc_abs_x),
    (AdcAbsY, 0x79, 2, adc_abs_y),
    (AdcIndirectX, 0x61, 1, adc_indirect_x),
    (AdcIndirectY, 0x71, 1, adc_indirect_y),

    (AndImm, 0x29, 1, and_imm),
    (AndZeroPage, 0x25, 1, and_zero_page),
    (AndZeroPageX, 0x35, 1, and_zero_page_x),
    (AndAbs, 0x2D, 2, and_abs),
    (AndAbsX, 0x3D, 2, and_abs_x),
    (AndAbsY, 0x39, 2, and_abs_y),
    (AndIndirectX, 0x21, 1, and_indirect_x),
    (AndIndirectY, 0x31, 1, and_indirect_y),

    (AslAcc, 0x0A, 0, asl_register_a),
    (AslZeroPage, 0x06, 1, asl_zero_page),
    (AslZeroPageX, 0x16, 1, asl_zero_page_x),
    (AslAbs, 0x0E, 2, asl_abs),
    (AslAbsX, 0x1E, 2, asl_abs_x),

    (Bcc, 0x90, 1, bcc_relative),

    (Bcs, 0xB0, 1, bcs_relative),

    (Beq, 0xF0, 1, beq_relative),

    (Bmi, 0x30, 1, bmi_relative),

    (Bne, 0xD0, 1, bne_relative),

    (Bpl, 0x10, 1, bpl_relative),

    (BitZeroPage, 0x24, 1, bit_zero_page),
    (BitAbs, 0x2C, 2, bit_abs),

    (Clc, 0x00, 0, clc_implied),

    (Cld, 0xD8, 0, cld_implied),

    (Cli, 0x58, 0, cli_implied),

    (Clv, 0xB8, 0, clv_implied),

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

    (Nop, 0xEA, 0, nop),

    (Sec, 0x38, 0, sec_implied),

    (Sed, 0xF8, 0, sed_implied),

    (Sei, 0x78, 0, sei_implied)
);
