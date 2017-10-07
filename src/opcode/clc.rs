use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn clc(registers: &mut Registers) {
    registers.set_carray_flag(false);
}

pub fn clc_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    clc(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn clc_when_carry_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_carray_flag(true);

        let expected_registers = Registers::new();

        clc(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn clc_when_carry_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = Registers::new();

        clc(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
