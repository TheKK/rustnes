use super::Cycle;

use opcode::utils::mem;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn adc(registers: &mut Registers, val: u8) {
    let a = registers.a;
    let carry = if registers.carry_flag() { 1 } else { 0 };
    let temp = val as u16 + a as u16 + carry;

    set_flag!(zero -> (registers, temp));
    set_flag!(sign -> (registers, temp));
    set_flag!(overflow -> (registers, a, val, temp));
    set_flag!(carry -> (registers, temp));

    registers.a = temp as u8;
}

opcode_fn_with_mode!(imm -> (adc_imm, adc, Cycle(2)));
opcode_fn_with_mode!(zero_page -> (adc_zero_page, adc, Cycle(3)));
opcode_fn_with_mode!(zero_page_x -> (adc_zero_page_x, adc, Cycle(4)));
opcode_fn_with_mode!(abs -> (adc_abs, adc, Cycle(4)));
opcode_fn_with_mode!(abs_x -> (adc_abs_x, adc,
                               page_crossed Cycle(5), or_else Cycle(4)));
opcode_fn_with_mode!(abs_y -> (adc_abs_y, adc,
                               page_crossed Cycle(5), or_else Cycle(4)));
opcode_fn_with_mode!(indirect_x -> (adc_indirect_x, adc, Cycle(6)));
opcode_fn_with_mode!(indirect_y -> (adc_indirect_y, adc,
                                    page_crossed Cycle(6), or_else Cycle(5)));

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
            reg.set_carry_flag(true);

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
            reg.set_carry_flag(true);
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
            reg.set_carry_flag(true);

            reg
        };

        adc(&mut registers, 0b10000001);

        assert_eq!(expected_registers, registers);
    }
}
