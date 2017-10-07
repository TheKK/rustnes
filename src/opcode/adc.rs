use super::Cycle;

use opcode::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn adc(registers: &mut Registers, val: u8) {
    let a = registers.a;
    let carry = if registers.carry_flag() { 1 } else { 0 };
    let temp = val as u16 + a as u16 + carry;

    let zero_flag = temp & 0xFF == 0x00;
    let sign_flag = (temp >> 7) & 1 == 1;
    let overflow_flag = (((a ^ val) & 0x80) == 0x00) && ((a as u16 ^ temp) & 0x80) > 0x00;
    let carry_flag = temp > 0xFF;

    registers.set_zero_flag(zero_flag);
    registers.set_sign_flag(sign_flag);
    registers.set_overflow_flag(overflow_flag);
    registers.set_carray_flag(carry_flag);

    registers.a = temp as u8;
}

pub fn adc_imm(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem::read_imm(&mem, pc);

    adc(registers, val);

    Cycle(2)
}

pub fn adc_zero_page(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_zero_page(&mem, registers.pc);

    adc(registers, val);

    Cycle(3)
}

pub fn adc_zero_page_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_zero_page_indexed(&mem, registers.pc, registers.x);

    adc(registers, val);

    Cycle(4)
}

pub fn adc_abs(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_abs(&mem, registers.pc);

    adc(registers, val);

    Cycle(4)
}

pub fn adc_abs_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let (val, page_crossed) = mem::read_abs_indexed(&mem, registers.pc, registers.x);

    adc(registers, val);

    match page_crossed {
        true => Cycle(5),
        false => Cycle(4),
    }
}

pub fn adc_abs_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let (val, page_crossed) = mem::read_abs_indexed(&mem, registers.pc, registers.y);

    adc(registers, val);

    match page_crossed {
        true => Cycle(5),
        false => Cycle(4),
    }
}

pub fn adc_indirect_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let val = mem::read_indirect_x(&mem, registers.pc, registers.x);

    adc(registers, val);

    Cycle(6)
}

pub fn adc_indirect_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let (val, page_crossed) = mem::read_indirect_y(&mem, registers.pc, registers.y);

    adc(registers, val);

    match page_crossed {
        true => Cycle(6),
        false => Cycle(5),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::Registers;

    #[test]
    fn adc_without_overflowing() {
        let mut registers = Registers::new();
        registers.a = 0x41;

        let expected_registers = {
            let mut reg = Registers::new();

            reg.a = 0x42;

            reg
        };

        adc(&mut registers, 0x01);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn adc_result_is_zero() {
        let mut registers = Registers::new();
        registers.a = 0b00000001;

        let expected_registers = {
            let mut reg = Registers::new();

            reg.a = 0x00;
            reg.set_zero_flag(true);
            reg.set_carray_flag(true);

            reg
        };

        adc(&mut registers, 0b11111111);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn adc_result_is_overflowing() {
        let mut registers = Registers::new();
        registers.a = 0x80;

        let expected_registers = {
            let mut reg = Registers::new();

            reg.a = 0x0;
            reg.set_zero_flag(true);
            reg.set_carray_flag(true);
            reg.set_overflow_flag(true);

            reg
        };

        adc(&mut registers, 0x80);

        assert_eq!(expected_registers, registers);
    }

    #[test]
    fn adc_result_is_carried() {
        let mut registers = Registers::new();
        registers.a = 0b01111111;

        let expected_registers = {
            let mut reg = Registers::new();

            reg.a = 0x00;
            reg.set_zero_flag(true);
            reg.set_carray_flag(true);

            reg
        };

        adc(&mut registers, 0b10000001);

        assert_eq!(expected_registers, registers);
    }
}
