use opcode;
use opcode::OpCode;

const MEM_ADDR_MAX: usize = 512;

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

        match opcode {
            OpCode::Nop => opcode::nop(&mut self.registers, &mut self.memory),
            OpCode::LdaImm => opcode::lda_imm(&mut self.registers, &mut self.memory),
            _ => unimplemented!(),
        }

        self.registers.pc += (1 + opcode.operands_num());
        self.current_cycles += opcode.cycles_num() as u32;
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    mod opcode {
        use super::*;

        #[test]
        fn lda_imm() {
            let mut cpu = RP2A03::new();
            cpu.memory.write(0, OpCode::LdaImm.into());
            cpu.memory.write(1, 0x42);

            let mem_snapshot = cpu.memory.clone();
            let regs_snaptshot = cpu.registers.clone();

            cpu.execute();

            assert_eq!(cpu.memory, mem_snapshot);
            assert_eq!(cpu.registers.a, 0x42);
            assert_eq!(cpu.registers.p, regs_snaptshot.p);
            assert_eq!(cpu.registers.sp, regs_snaptshot.sp);
            assert_eq!(cpu.registers.x, regs_snaptshot.x);
            assert_eq!(cpu.registers.y, regs_snaptshot.y);
        }

        #[test]
        fn nop() {
            let mut cpu = RP2A03::new();
            cpu.memory.write(0, OpCode::Nop.into());

            let mem_snapshot = cpu.memory.clone();
            let regs_snaptshot = cpu.registers.clone();

            cpu.execute();

            assert_eq!(cpu.memory, mem_snapshot);
            assert_eq!(cpu.registers.a, regs_snaptshot.a);
            assert_eq!(cpu.registers.p, regs_snaptshot.p);
            assert_eq!(cpu.registers.sp, regs_snaptshot.sp);
            assert_eq!(cpu.registers.x, regs_snaptshot.x);
            assert_eq!(cpu.registers.y, regs_snaptshot.y);
        }
    }
}
