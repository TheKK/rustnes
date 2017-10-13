use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn asl(registers: &mut Registers, val: u8) -> u8 {
    registers.set_carry_flag((val & 0b10000000) > 0);

    let new_val = val << 1;
    set_flag!(sign -> (registers, new_val));
    set_flag!(zero -> (registers, new_val));

    new_val
}

pub fn asl_register_a(registers: &mut Registers, _mem: &mut Memory) -> Cycle {
    let val = registers.a;

    let new_val = asl(registers, val);

    registers.a = new_val;

    Cycle(2)
}

opcode_fn_with_mode!(zero_page_memory -> (asl_zero_page, asl, Cycle(5)));
opcode_fn_with_mode!(zero_page_x_memory -> (asl_zero_page_x, asl, Cycle(6)));
opcode_fn_with_mode!(abs_memory -> (asl_abs, asl, Cycle(6)));
opcode_fn_with_mode!(abs_x_memory -> (asl_abs_x, asl, Cycle(7)));

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn asl_should_work() {
        let input_value = 0b00000001;

        let expected_registers = Registers::new();
        let expected_value = 0b00000010;

        let mut actual_registers = Registers::new();
        let actual_value = asl(&mut actual_registers, input_value);

        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn asl_result_is_carried() {
        let input_value = 0b10000001;

        let expected_registers = {
            let mut r = Registers::new();
            r.set_carry_flag(true);

            r
        };
        let expected_value = 0b00000010;

        let mut actual_registers = Registers::new();
        let actual_value = asl(&mut actual_registers, input_value);

        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn asl_result_is_zero() {
        let input_value = 0b10000000;

        let expected_registers = {
            let mut r = Registers::new();
            r.set_carry_flag(true);
            r.set_zero_flag(true);

            r
        };
        let expected_value = 0b00000000;

        let mut actual_registers = Registers::new();
        let actual_value = asl(&mut actual_registers, input_value);

        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn asl_result_is_sign() {
        let input_value = 0b01000000;

        let expected_registers = {
            let mut r = Registers::new();
            r.set_sign_flag(true);

            r
        };
        let expected_value = 0b10000000;

        let mut actual_registers = Registers::new();
        let actual_value = asl(&mut actual_registers, input_value);

        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_registers, expected_registers);
    }
}
