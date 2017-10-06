use super::Cycle;
use super::OpCode;

use cpu::Registers;
use cpu::Memory;

pub fn nop(_registers: &mut Registers, _mem: &mut Memory) -> Cycle {
    Cycle(2)
}

#[cfg(test)]
mod test {
    use super::*;

    use cpu::RP2A03;

    #[test]
    fn nop() {
        let mut cpu = RP2A03::new();
        cpu.memory.write(0, OpCode::Nop.into());

        let mem_snapshot = cpu.memory.clone();
        let regs_snaptshot = cpu.registers.clone();

        cpu.execute();

        assert_eq!(cpu.memory, mem_snapshot);
        assert_eq!(cpu.registers.p(), regs_snaptshot.p());
        assert_field_eq!(cpu.registers, regs_snaptshot, [a, sp, x, y]);
    }
}
