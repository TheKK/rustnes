use opcode::Cycle;
use opcode::OpCode;
use opcode::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn lda_assign_register_a(registers: &mut Registers, val: u8) {
    let zero_flag = val == 0;
    let sign_flag = (val >> 7) & 1 == 1;

    registers.a = val;
    registers.set_zero_flag(zero_flag);
    registers.set_sign_flag(sign_flag);
}

pub fn lda_imm(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_imm(&mem, pc);

    lda_assign_register_a(registers, val);

    Cycle(2)
}

pub fn lda_zero_page(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_zero_page(&mem, pc);

    lda_assign_register_a(registers, val);

    Cycle(3)
}

pub fn lda_zero_page_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let x = registers.x;
    let val = mem::read_zero_page_indexed(&mem, pc, x);

    lda_assign_register_a(registers, val);

    Cycle(4)
}

pub fn lda_abs(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_abs(&mem, pc);

    lda_assign_register_a(registers, val);

    Cycle(4)
}

pub fn lda_abs_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let x = registers.x;
    let pc = registers.pc;

    let (val, page_crossed) = mem::read_abs_indexed(&mem, pc, x);

    lda_assign_register_a(registers, val);

    match page_crossed {
        true => Cycle(5),
        false => Cycle(4),
    }
}

pub fn lda_abs_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let y = registers.y;
    let pc = registers.pc;
    let (val, page_crossed) = mem::read_abs_indexed(&mem, pc, y);

    lda_assign_register_a(registers, val);

    match page_crossed {
        true => Cycle(5),
        false => Cycle(4),
    }
}

pub fn lda_indirect_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let x = registers.x;
    let pc = registers.pc;

    let val = mem::read_indirect_x(&mem, pc, x);

    lda_assign_register_a(registers, val);

    Cycle(6)
}

pub fn lda_indirect_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let y = registers.y;
    let pc = registers.pc;

    let (val, page_crossed) = mem::read_indirect_y(&mem, pc, y);

    lda_assign_register_a(registers, val);

    match page_crossed {
        true => Cycle(6),
        false => Cycle(5),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::RP2A03;

    use opcode::utils::test::*;

    macro_rules! lda_test (
            (test_name=$test_name: ident,
             $opcode: expr,
             arrange_fn=$arrange_fn: expr,
             reg_a=$expected_reg_a: expr,
             zero_flag=$zero_flag: expr,
             sign_flag=$sign_flag: expr
             ) => {
                #[test]
                fn $test_name() {
                    let mut cpu = RP2A03::new();
                    cpu.memory.write(0, $opcode.into());
                    $arrange_fn(&mut cpu, $expected_reg_a);

                    let mem_snapshot = cpu.memory.clone();
                    let regs_snaptshot = cpu.registers.clone();

                    cpu.execute();

                    assert_eq!(cpu.memory, mem_snapshot);
                    assert_eq!(cpu.registers.a, $expected_reg_a);
                    assert_eq!(cpu.registers.zero_flag(), $zero_flag);
                    assert_eq!(cpu.registers.sign_flag(), $sign_flag);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [
                        carry_flag(),
                        interrupt_disable_flag(),
                        decimal_mode_flag(),
                        break_command_flag(),
                        overflow_flag()
                    ]);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [sp, x, y]);
                }
            }
        );

    lda_test!(
        test_name = lda_imm,
        OpCode::LdaImm,
        arrange_fn = arrange_for_imm,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_zero_page,
        OpCode::LdaZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_zero_page_x,
        OpCode::LdaZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs,
        OpCode::LdaAbs,
        arrange_fn = arrange_for_abs,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_x,
        OpCode::LdaAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_x_with_page_crossing,
        OpCode::LdaAbsX,
        arrange_fn = arrange_for_abs_x_with_page_crossing,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_y,
        OpCode::LdaAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_y_with_page_crossing,
        OpCode::LdaAbsY,
        arrange_fn = arrange_for_abs_y_with_page_crossing,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_indirect_x,
        OpCode::LdaIndirectX,
        arrange_fn = arrange_for_indirect_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_indirect_y,
        OpCode::LdaIndirectY,
        arrange_fn = arrange_for_indirect_y,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_indirect_y_with_page_crossed,
        OpCode::LdaIndirectY,
        arrange_fn = arrange_for_indirect_y_with_page_crossing,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_imm_zero,
        OpCode::LdaImm,
        arrange_fn = arrange_for_imm,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_zero_zero,
        OpCode::LdaZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_zero_page_x_zero,
        OpCode::LdaZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_zero,
        OpCode::LdaAbs,
        arrange_fn = arrange_for_abs,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_x_zero,
        OpCode::LdaAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_abs_y_zero,
        OpCode::LdaAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_indirect_x_zero,
        OpCode::LdaIndirectX,
        arrange_fn = arrange_for_indirect_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_indirect_y_zero,
        OpCode::LdaIndirectY,
        arrange_fn = arrange_for_indirect_y,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        test_name = lda_imm_sign,
        OpCode::LdaImm,
        arrange_fn = arrange_for_imm,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_zero_page_sign,
        OpCode::LdaZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_zero_page_x_sign,
        OpCode::LdaZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_abs_sign,
        OpCode::LdaAbs,
        arrange_fn = arrange_for_abs,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_abs_x_sign,
        OpCode::LdaAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_abs_y_sign,
        OpCode::LdaAbsY,
        arrange_fn = arrange_for_abs_y,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_indirect_x_sign,
        OpCode::LdaIndirectX,
        arrange_fn = arrange_for_indirect_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        test_name = lda_indirect_y_sign,
        OpCode::LdaIndirectY,
        arrange_fn = arrange_for_indirect_y,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );

    cross_boundary_cycle_count_add_one_test!(
        test_name = lda_abs_x_cycle_count,
        OpCode::LdaAbsX,
        no_boundary_crossing_arrange_fn = arrange_for_abs_x,
        boundary_crossing_arrange_fn = arrange_for_abs_x_with_page_crossing,
    );
    cross_boundary_cycle_count_add_one_test!(
        test_name = lda_abs_y_cycle_count,
        OpCode::LdaAbsY,
        no_boundary_crossing_arrange_fn = arrange_for_abs_y,
        boundary_crossing_arrange_fn = arrange_for_abs_y_with_page_crossing,
    );
    cross_boundary_cycle_count_add_one_test!(
        test_name = lda_indirect_y_cycle_count,
        OpCode::LdaIndirectY,
        no_boundary_crossing_arrange_fn = arrange_for_indirect_y,
        boundary_crossing_arrange_fn = arrange_for_indirect_y_with_page_crossing,
    );
}
