use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn clv(registers: &mut Registers) {
    registers.set_overflow_flag(false);
}

pub fn clv_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    clv(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn clv_when_overflow_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_overflow_flag(true);

        let expected_registers = Registers::new();

        clv(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn clv_when_carry_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = Registers::new();

        clv(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
