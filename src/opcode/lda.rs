use super::Cycle;
use super::OpCode;

use cpu::Registers;
use cpu::Memory;

#[inline]
fn lda_assign_register_a(registers: &mut Registers, val: u8) {
    let zero_flag = val == 0;
    let sign_flag = (val >> 7) & 1 == 1;

    registers.a = val;
    registers.set_zero_flag(zero_flag);
    registers.set_sign_flag(sign_flag);
}

pub fn lda_imm(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let val = mem.read((pc + 1) as u16);

    lda_assign_register_a(registers, val);

    Cycle(2)
}

pub fn lda_zero_page(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let addr = mem.read((pc + 1) as u16) as u16;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(3)
}

pub fn lda_zero_page_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let x = registers.x;
    let addr = (mem.read((pc + 1) as u16) + x) as u16;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(4)
}

pub fn lda_abs(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let pc = registers.pc;
    let addr_low = mem.read((pc + 1) as u16) as u16;
    let addr_high = mem.read((pc + 2) as u16) as u16;
    let addr = (addr_high << 8) + addr_low;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(4)
}

pub fn lda_abs_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let x = registers.x as u16;
    let pc = registers.pc;
    let addr_low = mem.read((pc + 1) as u16) as u16;
    let addr_high = mem.read((pc + 2) as u16) as u16;
    let addr = (addr_high << 8) + addr_low + x;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(4)
}

pub fn lda_abs_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let y = registers.y as u16;
    let pc = registers.pc;
    let addr_low = mem.read((pc + 1) as u16) as u16;
    let addr_high = mem.read((pc + 2) as u16) as u16;
    let addr = (addr_high << 8) + addr_low + y;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(4)
}

pub fn lda_indirect_x(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let x = registers.x as u16;
    let pc = registers.pc;

    let indirect_addr = mem.read((pc + 1) as u16) as u16 + x;

    let addr_low = mem.read(indirect_addr) as u16;
    let addr_high = mem.read(indirect_addr + 1) as u16;
    let addr = (addr_high << 8) + addr_low;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(6)
}

pub fn lda_indirect_y(registers: &mut Registers, mem: &mut Memory) -> Cycle {
    let y = registers.y as u16;
    let pc = registers.pc;

    let indirect_addr = mem.read((pc + 1) as u16) as u16;

    let addr_low = mem.read(indirect_addr) as u16;
    let addr_high = mem.read(indirect_addr + 1) as u16;
    let addr = (addr_high << 8) + addr_low + y;

    lda_assign_register_a(registers, mem.read(addr));

    Cycle(5)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::RP2A03;

    fn imm(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, val);
    }

    fn zero_page(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, 0x02);
        cpu.memory.write(2, val);
    }

    fn zero_page_x(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, 0x00);
        cpu.memory.write(2, val);

        cpu.registers.x = 0x02;
    }

    fn abs(cpu: &mut RP2A03, val: u8) {
        // ins $0102, note the order.
        cpu.memory.write(1, 0x02);
        cpu.memory.write(2, 0x01);
        cpu.memory.write(0x0102, val);
    }

    fn abs_x(cpu: &mut RP2A03, val: u8) {
        // ins $0401, note the order.
        cpu.memory.write(1, 0x01);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0402, val);

        cpu.registers.x = 0x01;
    }

    fn abs_y(cpu: &mut RP2A03, val: u8) {
        // ins $0401, note the order.
        cpu.memory.write(1, 0x01);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0402, val);

        cpu.registers.y = 0x01;
    }

    fn indirect_x(cpu: &mut RP2A03, val: u8) {
        // ins $1234, note the order.
        cpu.memory.write(1, 0x90);
        cpu.memory.write(0x0091, 0x34);
        cpu.memory.write(0x0092, 0x12);
        cpu.memory.write(0x1234, val);

        cpu.registers.x = 0x01;
    }

    fn indirect_y(cpu: &mut RP2A03, val: u8) {
        // ins $1234, note the order.
        cpu.memory.write(1, 0x90);
        cpu.memory.write(0x0090, 0x33);
        cpu.memory.write(0x0091, 0x12);
        cpu.memory.write(0x1234, val);

        cpu.registers.y = 0x01;
    }

    macro_rules! lda_test (
            ($test_name: ident,
             $opcode: expr,
             $arrange_fn: expr,
             reg_a=$expected_reg_a: expr,
             zero_flag=$zero_flag: expr,
             sign_flag=$sign_flag: expr
             ) => {
                #[test]
                fn $test_name() {
                    let mut cpu = RP2A03::new();
                    cpu.memory.write(0, $opcode.into());
                    $arrange_fn(&mut cpu, $expected_reg_a);

                    let mem_snapshot = cpu.memory.clone();
                    let regs_snaptshot = cpu.registers.clone();

                    cpu.execute();

                    assert_eq!(cpu.memory, mem_snapshot);
                    assert_eq!(cpu.registers.a, $expected_reg_a);
                    assert_eq!(cpu.registers.zero_flag(), $zero_flag);
                    assert_eq!(cpu.registers.sign_flag(), $sign_flag);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [sp, x, y]);
                }
            }
        );

    lda_test!(
        lda_imm,
        OpCode::LdaImm,
        imm,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_zero_page,
        OpCode::LdaZeroPage,
        zero_page,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_zero_page_x,
        OpCode::LdaZeroPageX,
        zero_page_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_abs,
        OpCode::LdaAbs,
        abs,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_abs_x,
        OpCode::LdaAbsX,
        abs_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_abs_y,
        OpCode::LdaAbsY,
        abs_y,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_indirect_x,
        OpCode::LdaIndirectX,
        indirect_x,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_indirect_y,
        OpCode::LdaIndirectY,
        indirect_y,
        reg_a = 0x42,
        zero_flag = false,
        sign_flag = false
    );
    lda_test!(
        lda_imm_zero,
        OpCode::LdaImm,
        imm,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_zero_zero,
        OpCode::LdaZeroPage,
        zero_page,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_zero_page_x_zero,
        OpCode::LdaZeroPageX,
        zero_page_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_abs_zero,
        OpCode::LdaAbs,
        abs,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_abs_x_zero,
        OpCode::LdaAbsX,
        abs_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_abs_y_zero,
        OpCode::LdaAbsY,
        abs_y,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_indirect_x_zero,
        OpCode::LdaIndirectX,
        indirect_x,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );
    lda_test!(
        lda_indirect_y_zero,
        OpCode::LdaIndirectY,
        indirect_y,
        reg_a = 0x00,
        zero_flag = true,
        sign_flag = false
    );

    lda_test!(
        lda_imm_sign,
        OpCode::LdaImm,
        imm,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_zero_page_sign,
        OpCode::LdaZeroPage,
        zero_page,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_zero_page_x_sign,
        OpCode::LdaZeroPageX,
        zero_page_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_abs_sign,
        OpCode::LdaAbs,
        abs,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_abs_x_sign,
        OpCode::LdaAbsX,
        abs_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_abs_y_sign,
        OpCode::LdaAbsY,
        abs_y,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_indirect_x_sign,
        OpCode::LdaIndirectX,
        indirect_x,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
    lda_test!(
        lda_indirect_y_sign,
        OpCode::LdaIndirectY,
        indirect_y,
        reg_a = 0b10000000,
        zero_flag = false,
        sign_flag = true
    );
}
