use opcode::OpCode;

const MEM_ADDR_MAX: usize = 512;

pub struct RP2A03 {
    memory: [u8; MEM_ADDR_MAX],

    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_p: u8,
    reg_pc: u8,
    reg_sp: u8,
}

impl RP2A03 {
    pub fn new() -> RP2A03 {
        RP2A03 {
            memory: [0x00; MEM_ADDR_MAX],

            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            reg_p: 0,
            reg_pc: 0,
            reg_sp: 0,
        }
    }

    pub fn execute(&mut self) {
        let pc = self.reg_pc;

        let opcode = OpCode::from(self.memory[pc as usize]);
        let operands_num = opcode.operands_num();
        let operands = &self.memory[(pc as usize)..((pc + operands_num) as usize)];

        self.reg_pc += operands_num;

        match opcode {
            OpCode::Nop => {}
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    mod opcode {
        use super::*;

        #[test]
        fn nop() {
            let mut cpu = RP2A03::new();
            cpu.memory[0] = OpCode::Nop.into();

            let old_mem = cpu.memory;
            let old_reg_a = cpu.reg_a;
            let old_reg_p = cpu.reg_p;
            let old_reg_sp = cpu.reg_sp;
            let old_reg_x = cpu.reg_x;
            let old_reg_y = cpu.reg_y;

            cpu.execute();

            assert_eq!(cpu.memory.as_ref(), old_mem.as_ref());
            assert_eq!(cpu.reg_a, old_reg_a);
            assert_eq!(cpu.reg_p, old_reg_p);
            assert_eq!(cpu.reg_sp, old_reg_sp);
            assert_eq!(cpu.reg_x, old_reg_x);
            assert_eq!(cpu.reg_y, old_reg_y);
        }
    }
}
