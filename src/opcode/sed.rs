use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn sed(registers: &mut Registers) {
    registers.set_decimal_mode_flag(true);
}

pub fn sed_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    sed(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn sed_when_decimal_mode_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_decimal_mode_flag(true);

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_decimal_mode_flag(true);

            reg
        };

        sed(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn sed_when_decimal_mode_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_decimal_mode_flag(true);

            reg
        };

        sed(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
