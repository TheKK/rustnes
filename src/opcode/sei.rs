use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn sei(registers: &mut Registers) {
    registers.set_interrupt_disable_flag(true);
}

pub fn sei_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    sei(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn sei_when_interrupt_disable_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_interrupt_disable_flag(true);

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_interrupt_disable_flag(true);

            reg
        };

        sei(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn sei_when_interrupt_disable_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_interrupt_disable_flag(true);

            reg
        };

        sei(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
