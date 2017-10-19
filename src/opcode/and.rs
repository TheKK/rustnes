use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn and(registers: &mut Registers, val: u8) {
    let a = registers.a;
    let result = val & a;

    set_flag!(sign -> (registers, result));
    set_flag!(zero -> (registers, result));

    registers.a = result;
}

opcode_fn_with_mode!(imm -> (and_imm, and, Cycle(2)));
opcode_fn_with_mode!(zero_page -> (and_zero_page, and, Cycle(3)));
opcode_fn_with_mode!(zero_page_x -> (and_zero_page_x, and, Cycle(4)));
opcode_fn_with_mode!(abs -> (and_abs, and, Cycle(4)));
opcode_fn_with_mode!(abs_x -> (and_abs_x, and,
                               page_crossed Cycle(5), or_else Cycle(4)));
opcode_fn_with_mode!(abs_y -> (and_abs_y, and,
                               page_crossed Cycle(5), or_else Cycle(4)));
opcode_fn_with_mode!(indirect_x -> (and_indirect_x, and, Cycle(6)));
opcode_fn_with_mode!(indirect_y -> (and_indirect_y, and,
                                    page_crossed Cycle(6), or_else Cycle(5)));

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn and_should_work() {
        let a = 0b00000101;
        let val = 0b00000111;
        let result = a & val;

        let mut actual_registers = {
            let mut r = Registers::new();
            r.a = a;

            r
        };

        let expected_register = {
            let mut r = Registers::new();
            r.a = result;

            r
        };

        and(&mut actual_registers, val);

        assert_eq!(expected_register, actual_registers);
    }

    #[test]
    fn and_result_is_zero() {
        let a = 0b00000010;
        let val = 0b00000101;
        let result = a & val;

        let mut actual_registers = {
            let mut r = Registers::new();
            r.a = a;

            r
        };

        let expected_register = {
            let mut r = Registers::new();
            r.a = result;
            r.set_zero_flag(true);

            r
        };

        and(&mut actual_registers, val);

        assert_eq!(expected_register, actual_registers);
    }

    #[test]
    fn and_result_is_negative() {
        let a = 0b10000010;
        let val = 0b10000101;
        let result = a & val;

        let mut actual_registers = {
            let mut r = Registers::new();
            r.a = a;

            r
        };

        let expected_register = {
            let mut r = Registers::new();
            r.a = result;
            r.set_sign_flag(true);

            r
        };

        and(&mut actual_registers, val);

        assert_eq!(expected_register, actual_registers);
    }
}
