use super::Cycle;
use super::utils;
use super::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn bvs(registers: &mut Registers, offset: u8) -> Cycle {
    let (new_pc, new_page) = utils::rel_addr(registers.pc, offset);

    match registers.overflow_flag() {
        true => {
            registers.pc = new_pc;

            if new_page { Cycle(4) } else { Cycle(3) }
        }
        false => Cycle(2),
    }
}

pub fn bvs_relative(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_rel(mem, registers);

    bvs(registers, val)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn bvs_with_positive_0_offset() {
        let input_value = 0b00000000;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 0;

            regs
        };

        let mut actual_registers = Registers::new();

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bvs_with_negtive_0_offset() {
        let input_value = 0b10000000;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 0;

            regs
        };

        let mut actual_registers = Registers::new();

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bvs_with_positive_127_offset_with_no_overflow_flag() {
        let input_value = 127u8;

        let expected_registers = Registers::new();

        let mut actual_registers = Registers::new();

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bvs_with_negtive_127_offset_with_no_overflow_flag() {
        let init_pc = 12345;
        let input_value = 0b11111111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_overflow_flag(false);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_overflow_flag(false);

            regs
        };

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bvs_with_positive_127_offset_with_overflow_flag() {
        let input_value = 127u8;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc += 127;

            regs.set_overflow_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();

            regs.set_overflow_flag(true);

            regs
        };

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }

    #[test]
    fn bvs_with_negtive_127_offset_with_overflow_flag() {
        let init_pc = 12345;
        let input_value = 0b11111111;

        let expected_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;
            regs.pc -= 127;

            regs.set_overflow_flag(true);

            regs
        };

        let mut actual_registers = {
            let mut regs = Registers::new();
            regs.pc = init_pc;

            regs.set_overflow_flag(true);

            regs
        };

        let _cycle_num = bvs(&mut actual_registers, input_value);

        assert_eq!(actual_registers, expected_registers);
    }
}
