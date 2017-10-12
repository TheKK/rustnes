use super::Cycle;
use super::OpCode;

use opcode::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn ldx(registers: &mut Registers, val: u8) {
    set_flag!(zero -> (registers, val));
    set_flag!(sign -> (registers, val));

    registers.x = val;
}

pub fn ldx_imm(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_imm(&mem, pc);

    ldx(registers, val);

    Cycle(2)
}

pub fn ldx_zero_page(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_zero_page(&mem, pc);

    ldx(registers, val);

    Cycle(3)
}

pub fn ldx_zero_page_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let y = registers.y;
    let val = mem::read_zero_page_indexed(&mem, pc, y);

    ldx(registers, val);

    Cycle(4)
}

pub fn ldx_abs(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_abs(&mem, pc);

    ldx(registers, val);

    Cycle(4)
}

pub fn ldx_abs_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let y = registers.y;
    let pc = registers.pc;
    let (val, page_crossed) = mem::read_abs_indexed(&mem, pc, y);

    ldx(registers, val);

    match page_crossed {
        true => Cycle(5),
        false => Cycle(4),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::RP2A03;

    use opcode::utils::test::*;

    macro_rules! ldx_test (
            (test_name=$test_name: ident,
             $opcode: expr,
             arrange_fn=$arrange_fn: expr,
             reg_x=$expected_reg_x: expr,
             zero_flag=$zero_flag: expr,
             sign_flag=$sign_flag: expr
             ) => {
                #[test]
                fn $test_name() {
                    let mut cpu = RP2A03::new();
                    cpu.memory.write(0, $opcode.into());
                    $arrange_fn(&mut cpu, $expected_reg_x);

                    let mem_snapshot = cpu.memory.clone();
                    let regs_snaptshot = cpu.registers.clone();

                    cpu.execute();

                    assert_eq!(cpu.memory, mem_snapshot);
                    assert_eq!(cpu.registers.x, $expected_reg_x);
                    assert_eq!(cpu.registers.zero_flag(), $zero_flag);
                    assert_eq!(cpu.registers.sign_flag(), $sign_flag);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [
                        carry_flag(),
                        interrupt_disable_flag(),
                        decimal_mode_flag(),
                        break_command_flag(),
                        overflow_flag()
                    ]);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [sp, a, y]);
                }
            }
        );

    ldx_test!(
        test_name = ldx_imm,
        OpCode::LdxImm,
        arrange_fn = arrange_for_imm,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_zero_page,
        OpCode::LdxZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_zero_page_y,
        OpCode::LdxZeroPageY,
        arrange_fn = arrange_for_zero_page_y,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_abs,
        OpCode::LdxAbs,
        arrange_fn = arrange_for_abs,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_abs_y,
        OpCode::LdxAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_abs_y_with_page_crossing,
        OpCode::LdxAbsY,
        arrange_fn = arrange_for_abs_y_with_page_crossing,
        reg_x = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_imm_zero,
        OpCode::LdxImm,
        arrange_fn = arrange_for_imm,
        reg_x = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_zero_zero,
        OpCode::LdxZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_x = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldx_test!(
        test_name = lda_zero_page_y_zero,
        OpCode::LdxZeroPageY,
        arrange_fn = arrange_for_zero_page_y,
        reg_x = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_abs_zero,
        OpCode::LdxAbs,
        arrange_fn = arrange_for_abs,
        reg_x = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_abs_y_zero,
        OpCode::LdxAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_x = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldx_test!(
        test_name = ldx_imm_sign,
        OpCode::LdxImm,
        arrange_fn = arrange_for_imm,
        reg_x = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldx_test!(
        test_name = ldx_zero_page_sign,
        OpCode::LdxZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_x = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldx_test!(
        test_name = ldx_zero_page_y_sign,
        OpCode::LdxZeroPageY,
        arrange_fn = arrange_for_zero_page_y,
        reg_x = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldx_test!(
        test_name = ldx_abs_sign,
        OpCode::LdxAbs,
        arrange_fn = arrange_for_abs,
        reg_x = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldx_test!(
        test_name = ldx_abs_y_sign,
        OpCode::LdxAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_x = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );

    cross_boundary_cycle_count_add_one_test!(
        test_name = ldx_abs_y_cycle_count,
        OpCode::LdxAbsY,
        no_boundary_crossing_arrange_fn = arrange_for_abs_y,
        boundary_crossing_arrange_fn = arrange_for_abs_y_with_page_crossing,
    );
}
