use super::Cycle;
use super::utils;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn bit(registers: &mut Registers, val: u8) {
    #[inline]
    fn bit_6(val: u8) -> bool {
        0b01000000 & val > 0
    }

    let a = registers.a;

    registers.set_sign_flag(utils::is_sign(val));
    registers.set_overflow_flag(bit_6(val));
    registers.set_zero_flag(val & a == 0);
}

opcode_fn_with_mode!(zero_page -> (bit_zero_page, bit, Cycle(3)));
opcode_fn_with_mode!(abs -> (bit_abs, bit, Cycle(4)));

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn bit_with_no_flag_set() {
        let a = 0b00000101;
        let val = 0b00000111;

        let expected_registers = {
            let mut r = Registers::new();
            r.a = a;

            r
        };

        let mut actual_registers = expected_registers.clone();

        bit(&mut actual_registers, val);

        assert_eq!(expected_registers, actual_registers);
    }

    #[test]
    fn bit_with_zero_flag_set() {
        let a = 0b00000101;
        let val = 0b00000010;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs.set_zero_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs
        };

        bit(&mut actual_registers, val);

        assert_eq!(expected_registers, actual_registers);
    }

    #[test]
    fn bit_with_overflow_flag_set() {
        let a = 0b00000101;
        let val = 0b01000111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs.set_overflow_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs
        };

        bit(&mut actual_registers, val);

        assert_eq!(expected_registers, actual_registers);
    }

    #[test]
    fn bit_with_sign_flag_set() {
        let a = 0b10000101;
        let val = 0b10000111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs.set_sign_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.a = a;

            regs
        };

        bit(&mut actual_registers, val);

        assert_eq!(expected_registers, actual_registers);
    }
}
