use super::Cycle;
use super::OpCode;

use opcode::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn ldy(registers: &mut Registers, val: u8) {
    set_flag!(zero -> (registers, val));
    set_flag!(sign -> (registers, val));

    registers.y = val;
}

opcode_fn_with_mode!(imm -> (ldy_imm, ldy, Cycle(2)));
opcode_fn_with_mode!(zero_page -> (ldy_zero_page, ldy, Cycle(3)));
opcode_fn_with_mode!(zero_page_x -> (ldy_zero_page_x, ldy, Cycle(4)));
opcode_fn_with_mode!(abs -> (ldy_abs, ldy, Cycle(4)));
opcode_fn_with_mode!(abs_x -> (ldy_abs_x, ldy,
                               page_crossed Cycle(5), or_else Cycle(4)));

#[cfg(test)]
mod test {
    use super::*;

    use cpu::RP2A03;

    use opcode::utils::test::*;

    macro_rules! ldy_test (
            (test_name=$test_name: ident,
             $opcode: expr,
             arrange_fn=$arrange_fn: expr,
             reg_y=$expected_reg_y: expr,
             zero_flag=$zero_flag: expr,
             sign_flag=$sign_flag: expr
             ) => {
                #[test]
                fn $test_name() {
                    let mut cpu = RP2A03::new();
                    cpu.memory.write(0, $opcode.into());
                    $arrange_fn(&mut cpu, $expected_reg_y);

                    let mem_snapshot = cpu.memory.clone();
                    let regs_snaptshot = cpu.registers.clone();

                    cpu.execute();

                    assert_eq!(cpu.memory, mem_snapshot);
                    assert_eq!(cpu.registers.y, $expected_reg_y);
                    assert_eq!(cpu.registers.zero_flag(), $zero_flag);
                    assert_eq!(cpu.registers.sign_flag(), $sign_flag);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [
                        carry_flag(),
                        interrupt_disable_flag(),
                        decimal_mode_flag(),
                        break_command_flag(),
                        overflow_flag()
                    ]);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [sp, a, x]);
                }
            }
        );

    #[test]
    fn ldy_should_work() {
        let mut actual_registers = Registers::new();

        let expected_val = 0x42;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.y = expected_val;

            reg_clone
        };

        ldy(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

    #[test]
    fn ldy_with_zero_value() {
        let mut actual_registers = Registers::new();

        let expected_val = 0x00;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.y = expected_val;
            reg_clone.set_zero_flag(true);

            reg_clone
        };

        ldy(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

    #[test]
    fn ldy_with_sign_value() {
        let mut actual_registers = Registers::new();

        let expected_val = 0b10000000;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.y = expected_val;
            reg_clone.set_sign_flag(true);

            reg_clone
        };

        ldy(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

    ldy_test!(
        test_name = ldy_imm,
        OpCode::LdyImm,
        arrange_fn = arrange_for_imm,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_zero_page,
        OpCode::LdyZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_zero_page_x,
        OpCode::LdyZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_abs,
        OpCode::LdyAbs,
        arrange_fn = arrange_for_abs,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_abs_x,
        OpCode::LdyAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_abs_x_with_page_crossing,
        OpCode::LdyAbsX,
        arrange_fn = arrange_for_abs_x_with_page_crossing,
        reg_y = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_imm_zero,
        OpCode::LdyImm,
        arrange_fn = arrange_for_imm,
        reg_y = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_zero_zero,
        OpCode::LdyZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_y = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_zero_page_x_zero,
        OpCode::LdyZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_y = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_abs_zero,
        OpCode::LdyAbs,
        arrange_fn = arrange_for_abs,
        reg_y = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_abs_x_zero,
        OpCode::LdyAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_y = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    ldy_test!(
        test_name = ldy_imm_sign,
        OpCode::LdyImm,
        arrange_fn = arrange_for_imm,
        reg_y = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldy_test!(
        test_name = ldy_zero_page_sign,
        OpCode::LdyZeroPage,
        arrange_fn = arrange_for_zero_page,
        reg_y = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldy_test!(
        test_name = ldy_zero_page_x_sign,
        OpCode::LdyZeroPageX,
        arrange_fn = arrange_for_zero_page_x,
        reg_y = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldy_test!(
        test_name = ldy_abs_sign,
        OpCode::LdyAbs,
        arrange_fn = arrange_for_abs,
        reg_y = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    ldy_test!(
        test_name = ldy_abs_x_sign,
        OpCode::LdyAbsX,
        arrange_fn = arrange_for_abs_x,
        reg_y = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );

    cross_boundary_cycle_count_add_one_test!(
        test_name = ldy_abs_x_cycle_count,
        OpCode::LdyAbsX,
        no_boundary_crossing_arrange_fn = arrange_for_abs_x,
        boundary_crossing_arrange_fn = arrange_for_abs_x_with_page_crossing,
    );
}
