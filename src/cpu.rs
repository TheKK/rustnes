use opcode::OpCode;

const MEM_ADDR_MAX: usize = 0xffff;

#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub pc: u8,
    pub sp: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    raw_memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { raw_memory: vec![0x00; MEM_ADDR_MAX] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.raw_memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.raw_memory[addr as usize] = val;
    }
}

pub struct RP2A03 {
    memory: Memory,
    registers: Registers,
    current_cycles: u32,
}

impl RP2A03 {
    pub fn new() -> RP2A03 {
        RP2A03 {
            memory: Memory::new(),
            registers: Registers {
                a: 0x00,
                x: 0x00,
                y: 0x00,
                p: 0x00,
                pc: 0x00,
                sp: 0x00,
            },
            current_cycles: 0,
        }
    }

    pub fn execute(&mut self) {
        let pc = self.registers.pc;

        let opcode = OpCode::from(self.memory.read(pc as u16));
        let opcode_fn = opcode.get_fn();

        opcode_fn(&mut self.registers, &mut self.memory);

        self.registers.pc += (1 + opcode.operands_num());
        self.current_cycles += opcode.cycles_num() as u32;
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    mod opcode {
        use super::*;

        fn imm(cpu: &mut RP2A03) {
            cpu.memory.write(1, 0x42);
        }

        fn zero_page(cpu: &mut RP2A03) {
            cpu.memory.write(1, 0x02);
            cpu.memory.write(2, 0x42);
        }

        fn zero_page_x(cpu: &mut RP2A03) {
            cpu.memory.write(1, 0x00);
            cpu.memory.write(2, 0x42);

            cpu.registers.x = 0x02;
        }

        fn abs(cpu: &mut RP2A03) {
            // ins $0102, note the order.
            cpu.memory.write(1, 0x02);
            cpu.memory.write(2, 0x01);
            cpu.memory.write(0x0102, 0x42);
        }

        macro_rules! assert_field_eq (
            ($left: expr, $right: expr, [$($field: ident), *]) => {
                $(
                    assert_eq!($left.$field, $right.$field);
                )*
            }
        );

        macro_rules! lda_test (
            ($test_name: ident, $opcode: expr, $arrange_fn: expr) => {
                #[test]
                fn $test_name() {
                    let mut cpu = RP2A03::new();
                    cpu.memory.write(0, $opcode.into());
                    $arrange_fn(&mut cpu);

                    let mem_snapshot = cpu.memory.clone();
                    let regs_snaptshot = cpu.registers.clone();

                    cpu.execute();

                    assert_eq!(cpu.memory, mem_snapshot);
                    assert_eq!(cpu.registers.a, 0x42);
                    assert_field_eq!(cpu.registers, regs_snaptshot, [p, sp, x, y]);
                }
            }
        );

        lda_test!(lda_imm, OpCode::LdaImm, imm);
        lda_test!(lda_zero_page, OpCode::LdaZeroPage, zero_page);
        lda_test!(lda_zero_page_x, OpCode::LdaZeroPageX, zero_page_x);
        lda_test!(lda_abs, OpCode::LdaAbs, abs);

        #[test]
        fn nop() {
            let mut cpu = RP2A03::new();
            cpu.memory.write(0, OpCode::Nop.into());

            let mem_snapshot = cpu.memory.clone();
            let regs_snaptshot = cpu.registers.clone();

            cpu.execute();

            assert_eq!(cpu.memory, mem_snapshot);
            assert_field_eq!(cpu.registers, regs_snaptshot, [a, p, sp, x, y]);
        }
    }
}
