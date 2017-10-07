use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn cld(registers: &mut Registers) {
    registers.set_decimal_mode_flag(false);
}

pub fn cld_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    cld(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn cld_when_carry_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_decimal_mode_flag(true);

        let expected_registers = Registers::new();

        cld(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn cld_when_carry_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = Registers::new();

        cld(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
