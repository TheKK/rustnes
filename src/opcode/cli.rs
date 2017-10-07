use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn cli(registers: &mut Registers) {
    registers.set_interrupt_disable_flag(false);
}

pub fn cli_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    cli(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn cli_when_carry_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_interrupt_disable_flag(true);

        let expected_registers = Registers::new();

        cli(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn cli_when_carry_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = Registers::new();

        cli(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
