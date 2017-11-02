use super::Cycle;
use super::utils;
use super::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn bne(registers: &mut Registers, offset: u8) -> Cycle {
    let (new_pc, new_page) = utils::rel_addr(registers.pc, offset);

    match registers.zero_flag() {
        true => Cycle(2),
        false => {
            registers.pc = new_pc;

            if new_page { Cycle(4) } else { Cycle(3) }
        }
    }
}

pub fn bne_relative(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_rel(mem, registers);

    bne(registers, val)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn bne_with_positive_0_offset() {
        let input_value = 0b00000000;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 0;

            regs.set_zero_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();

            regs.set_zero_flag(true);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bne_with_negtive_0_offset() {
        let input_value = 0b10000000;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 0;

            regs.set_zero_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();

            regs.set_zero_flag(true);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bne_with_positive_127_offset_with_zero_flag() {
        let input_value = 127u8;

        let expected_registers = {
            let mut regs = Registers::new();

            regs.set_zero_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();

            regs.set_zero_flag(true);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bne_with_negtive_127_offset_with_zero_flag() {
        let init_pc = 12345;
        let input_value = 0b11111111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_zero_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_zero_flag(true);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bne_with_positive_127_offset_with_no_zero_flag() {
        let input_value = 127u8;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 127;

            regs.set_zero_flag(false);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();

            regs.set_zero_flag(false);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bne_with_negtive_127_offset_with_no_zero_flag() {
        let init_pc = 12345;
        let input_value = 0b11111111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;
            regs.pc -= 127;

            regs.set_zero_flag(false);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_zero_flag(false);

            regs
        };

        let _cycle_num = bne(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }
}
