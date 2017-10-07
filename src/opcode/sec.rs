use super::Cycle;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn sec(registers: &mut Registers) {
    registers.set_carry_flag(true);
}

pub fn sec_implied(registers: &mut Registers, _: &mut Memory) -> Cycle {
    sec(registers);

    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn sec_when_carry_flag_is_set() {
        let mut registers = Registers::new();
        registers.set_carry_flag(true);

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_carry_flag(true);

            reg
        };

        sec(&mut registers);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn sec_when_carry_flag_is_not_set() {
        let mut registers = Registers::new();

        let expected_registers = {
            let mut reg = Registers::new();
            reg.set_carry_flag(true);

            reg
        };

        sec(&mut registers);

        assert_eq!(expected_registers, registers);
    }
}
