use super::Cycle;
use super::OpCode;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn ldx(registers: &mut Registers, val: u8) {
    set_flag!(zero -> (registers, val));
    set_flag!(sign -> (registers, val));

    registers.x = val;
}

opcode_fn_with_mode!(imm -> (ldx_imm, ldx, Cycle(2)));
opcode_fn_with_mode!(zero_page -> (ldx_zero_page, ldx, Cycle(3)));
opcode_fn_with_mode!(zero_page_y -> (ldx_zero_page_y, ldx, Cycle(4)));
opcode_fn_with_mode!(abs -> (ldx_abs, ldx, Cycle(4)));
opcode_fn_with_mode!(abs_y -> (ldx_abs_y, ldx,
                               page_crossed Cycle(5), or_else Cycle(4)));

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

    #[test]
    fn ldx_should_work() {
        let mut actual_registers = Registers::new();

        let expected_val = 0x42;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.x = expected_val;

            reg_clone
        };

        ldx(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

    #[test]
    fn ldx_with_zero_value() {
        let mut actual_registers = Registers::new();

        let expected_val = 0x00;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.x = expected_val;
            reg_clone.set_zero_flag(true);

            reg_clone
        };

        ldx(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

    #[test]
    fn ldx_with_sign_value() {
        let mut actual_registers = Registers::new();

        let expected_val = 0b10000000;
        let expected_registers = {
            let mut reg_clone = actual_registers.clone();
            reg_clone.x = expected_val;
            reg_clone.set_sign_flag(true);

            reg_clone
        };

        ldx(&mut actual_registers, expected_val);

        assert_eq!(actual_registers, expected_registers)
    }

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
        test_name = ldx_zero_page_y_zero,
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
