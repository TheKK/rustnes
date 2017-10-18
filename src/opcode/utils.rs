#[inline]
fn new_page(old_pc: u16, new_pc: u16) -> bool {
    (old_pc & 0xFF00) != (new_pc & 0xFF00)
}

// TODO Use math magic to make this faster!!
// Returned tuple means: (new_pc, page_crossed)
#[inline]
pub fn rel_addr(pc: u16, val: u8) -> (u16, bool) {
    let unsigned_val = ((val << 1) >> 1) as u16;
    let negtive = (val & 0b10000000) > 0;

    let new_pc = match negtive {
        true => pc - unsigned_val,
        false => pc + unsigned_val,
    };

    (new_pc, new_page(pc, new_pc))
}

#[macro_export]
macro_rules! assert_field_eq (
    ($left: expr, $right: expr, [$($field: ident), *]) => {
        $(
            assert_eq!($left.$field, $right.$field);
        )*
    };

    ($left: expr, $right: expr, [$($field: ident()), *]) => {
        $(
            assert_eq!($left.$field(), $right.$field());
        )*
    };
);

#[macro_export]
macro_rules! cross_boundary_cycle_count_add_one_test (
    (test_name=$test_name: ident,
     $opcode: expr,
     no_boundary_crossing_arrange_fn=$no_boundary_crossing_arrange_fn: expr,
     boundary_crossing_arrange_fn=$boundary_crossing_arrange_fn: expr,
    ) => {
        #[test]
        fn $test_name() {
            let Cycle(cycle_without_page_crossing) = {
                let mut cpu = RP2A03::new();
                cpu.memory.write(0, $opcode.into());
                $no_boundary_crossing_arrange_fn(&mut cpu, 0x42);

                cpu.execute()
            };

            let Cycle(cycle_with_page_crossing) = {
                let mut cpu = RP2A03::new();
                cpu.memory.write(0, $opcode.into());
                $boundary_crossing_arrange_fn(&mut cpu, 0x42);

                cpu.execute()
            };

            assert_eq!(cycle_with_page_crossing,
                       cycle_without_page_crossing + 1);
        }
    }
);

#[macro_export]
macro_rules! set_flag(
    (zero -> ($registers: expr, $val: expr)) => {
        $registers.set_zero_flag($val & 0xFF == 0x00);
    };

    (sign -> ($registers: expr, $val: expr)) => {
        $registers.set_sign_flag(($val >> 7) & 1 == 1);
    };

    (overflow -> ($registers: expr, $operand_a: expr, $operand_b: expr, $result: expr)) => {
        let flag = ((($operand_a ^ $operand_b) & 0x80) == 0x00) &&
            (($operand_a as u16 ^ $result) & 0x80) > 0x00;

        $registers.set_overflow_flag(flag);
    };

    (carry -> ($registers: expr, $val: expr)) => {
        $registers.set_carry_flag($val > 0xFF);
    };
);

macro_rules! gen_opcode_fn {
    ($fn_name: ident, $instruction: expr, $cycles_num: expr, $addressing_mode: expr) => {

        pub fn $fn_name(registers: &mut Registers, mem: &mut Memory) -> Cycle {
            let val = $addressing_mode(mem, registers);

            $instruction(registers, val);

            $cycles_num
        }
    };

    ($fn_name: ident, $instruction: expr,
     page_crossed $page_crossed_cycles_num: expr,
     or_else $normal_cycles_num: expr,
     $addressing_mode: expr) => {

        pub fn $fn_name(registers: &mut Registers, mem: &mut Memory) -> Cycle {
            let (val, page_crossed) = $addressing_mode(mem, registers);

            $instruction(registers, val);

            match page_crossed {
                true => $page_crossed_cycles_num,
                false => $normal_cycles_num,
            }
        }
    };


    (addr ->
     $fn_name: ident, $instruction: expr, $cycles_num: expr, $addressing_mode: expr) => {

        pub fn $fn_name(registers: &mut Registers, mem: &mut Memory) -> Cycle {
            let addr = $addressing_mode(mem, registers);
            let val = mem.read(addr);

            let result = $instruction(registers, val);
            mem.write(addr, result);

            $cycles_num
        }
    };


    (addr_ignore_page_crossing ->
     $fn_name: ident, $instruction: expr, $cycles_num: expr, $addressing_mode: expr) => {

        pub fn $fn_name(registers: &mut Registers, mem: &mut Memory) -> Cycle {
            let (addr, _page_crossed) = $addressing_mode(mem, registers);
            let val = mem.read(addr);

            let result = $instruction(registers, val);
            mem.write(addr, result);

            $cycles_num
        }
    };
}

#[macro_export]
macro_rules! opcode_fn_with_mode(
    (acc -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        pub fn $fn_name(registers: &mut Registers, mem: &mut Memory) -> Cycle {
            $instruction(registers, old_a);

            $cycles_num
        }
    };

    (zero_page_memory -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!(addr -> $fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::get_zero_page_addr);

    };

    (zero_page_x_memory -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!(addr -> $fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::get_zero_page_x_addr);
    };

    (abs_memory -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!(addr -> $fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::get_abs_addr);
    };

    (abs_memory -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!(addr -> $fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::get_abs_addr);
    };

    (abs_x_memory -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!(addr_ignore_page_crossing ->
                       $fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::get_abs_x_addr);
    };

    (imm -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_imm);
    };

    (zero_page -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_zero_page);
    };

    (zero_page_x -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_zero_page_x);
    };

    (zero_page_y -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_zero_page_y);
    };

    (rel -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_rel);
    };

    (abs -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_abs);
    };

    (abs_x -> ($fn_name: ident, $instruction: expr,
               page_crossed $page_crossed_cycles_num: expr,
               or_else $normal_cycles_num: expr)) => {

        gen_opcode_fn!($fn_name, $instruction,
                       page_crossed $page_crossed_cycles_num,
                       or_else $normal_cycles_num,
                       $crate::opcode::utils::mem::read_abs_x);
    };

    (abs_y -> ($fn_name: ident, $instruction: expr,
               page_crossed $page_crossed_cycles_num: expr,
               or_else $normal_cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction,
                       page_crossed $page_crossed_cycles_num,
                       or_else $normal_cycles_num,
                       $crate::opcode::utils::mem::read_abs_y);
    };

    (indirect_x -> ($fn_name: ident, $instruction: expr, $cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction, $cycles_num,
                       $crate::opcode::utils::mem::read_indirect_x);
    };

    (indirect_y -> ($fn_name: ident, $instruction: expr,
                    page_crossed $page_crossed_cycles_num: expr,
                    or_else $normal_cycles_num: expr)) => {
        gen_opcode_fn!($fn_name, $instruction,
                       page_crossed $page_crossed_cycles_num,
                       or_else $normal_cycles_num,
                       $crate::opcode::utils::mem::read_indirect_y);
    };
);

pub mod mem {
    use cpu::Memory;
    use cpu::Registers;

    #[inline]
    fn compose_addr(addr_high: u8, addr_low: u8) -> u16 {
        ((addr_high as u16) << 8) + addr_low as u16
    }

    #[inline]
    fn compose_indexed_addr(addr_high: u8, addr_low: u8, index: u8) -> (u16, bool) {
        let (addr, page_crossed) = match addr_low.overflowing_add(index) {
            (addr_low, true) => {
                let (addr_high, _overflowed) = addr_high.overflowing_add(1);

                (compose_addr(addr_high, addr_low), true)
            }
            (addr_low, false) => (compose_addr(addr_high, addr_low), false),
        };

        (addr, page_crossed)
    }

    #[inline]
    pub fn get_imm_addr(_mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;

        (pc + 1) as u16
    }

    #[inline]
    pub fn read_imm(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_imm_addr(mem, registers))
    }

    #[inline]
    pub fn get_zero_page_addr(mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;
        let addr = mem.read((pc + 1) as u16) as u16;

        addr
    }

    #[inline]
    pub fn read_zero_page(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_zero_page_addr(mem, registers))
    }

    #[inline]
    pub fn get_zero_page_x_addr(mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;
        let x = registers.x;
        let base_addr = mem.read((pc + 1) as u16);
        let indexed_addr = (base_addr + x) as u16;

        indexed_addr
    }

    #[inline]
    pub fn read_zero_page_x(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_zero_page_x_addr(mem, registers))
    }

    #[inline]
    pub fn get_zero_page_y_addr(mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;
        let y = registers.y;
        let base_addr = mem.read((pc + 1) as u16);
        let indexed_addr = (base_addr + y) as u16;

        indexed_addr
    }

    #[inline]
    pub fn read_zero_page_y(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_zero_page_y_addr(mem, registers))
    }

    #[inline]
    pub fn read_rel(mem: &Memory, registers: &Registers) -> u8 {
        let pc = registers.pc;

        mem.read((pc + 1) as u16)
    }

    #[inline]
    pub fn get_abs_addr(mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;
        let addr_low = mem.read((pc + 1) as u16);
        let addr_high = mem.read((pc + 2) as u16);
        let addr = compose_addr(addr_high, addr_low);

        addr
    }

    #[inline]
    pub fn read_abs(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_abs_addr(mem, registers))
    }

    #[inline]
    pub fn get_abs_x_addr(mem: &Memory, registers: &Registers) -> (u16, bool) {
        let pc = registers.pc;
        let x = registers.x;
        let addr_low = mem.read((pc + 1) as u16);
        let addr_high = mem.read((pc + 2) as u16);

        compose_indexed_addr(addr_high, addr_low, x)
    }

    #[inline]
    pub fn read_abs_x(mem: &Memory, registers: &Registers) -> (u8, bool) {
        let (addr, page_crossed) = get_abs_x_addr(mem, registers);

        (mem.read(addr), page_crossed)
    }

    #[inline]
    pub fn get_abs_y_addr(mem: &Memory, registers: &Registers) -> (u16, bool) {
        let pc = registers.pc;
        let y = registers.y;
        let addr_low = mem.read((pc + 1) as u16);
        let addr_high = mem.read((pc + 2) as u16);

        compose_indexed_addr(addr_high, addr_low, y)
    }

    #[inline]
    pub fn read_abs_y(mem: &Memory, registers: &Registers) -> (u8, bool) {
        let (addr, page_crossed) = get_abs_y_addr(mem, registers);

        (mem.read(addr), page_crossed)
    }

    #[inline]
    pub fn get_indirect_x_addr(mem: &Memory, registers: &Registers) -> u16 {
        let pc = registers.pc;
        let x = registers.x;
        let indirect_addr = mem.read((pc + 1) as u16) + x;

        // TODO Figure out how this hardware handle one byte address overflowing.
        let indirect_addr = indirect_addr as u16;

        let addr_low = mem.read(indirect_addr);
        let addr_high = mem.read(indirect_addr + 1);
        let addr = compose_addr(addr_high, addr_low);

        addr
    }

    #[inline]
    pub fn read_indirect_x(mem: &Memory, registers: &Registers) -> u8 {
        mem.read(get_indirect_x_addr(mem, registers))
    }

    #[inline]
    pub fn get_indirect_y_addr(mem: &Memory, registers: &Registers) -> (u16, bool) {
        let pc = registers.pc;
        let y = registers.y;
        let indirect_addr = mem.read((pc + 1) as u16) as u16;

        let addr_low = mem.read(indirect_addr);
        let addr_high = mem.read(indirect_addr + 1);

        compose_indexed_addr(addr_high, addr_low, y)
    }

    #[inline]
    pub fn read_indirect_y(mem: &Memory, registers: &Registers) -> (u8, bool) {
        let (addr, page_crossed) = get_indirect_y_addr(mem, registers);

        (mem.read(addr), page_crossed)
    }
}

#[cfg(test)]
pub mod test {
    use opcode::Cycle;

    use cpu::Memory;
    use cpu::RP2A03;
    use cpu::Registers;

    pub fn arrange_for_imm(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, val);
    }

    pub fn arrange_for_zero_page(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, 0x02);
        cpu.memory.write(2, val);
    }

    pub fn arrange_for_zero_page_x(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, 0x00);
        cpu.memory.write(2, val);

        cpu.registers.x = 0x02;
    }

    pub fn arrange_for_zero_page_y(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, 0x00);
        cpu.memory.write(2, val);

        cpu.registers.y = 0x02;
    }

    pub fn arrange_for_rel(cpu: &mut RP2A03, val: u8) {
        cpu.memory.write(1, val);
    }

    pub fn arrange_for_abs(cpu: &mut RP2A03, val: u8) {
        // ins $0102, note the order.
        cpu.memory.write(1, 0x02);
        cpu.memory.write(2, 0x01);
        cpu.memory.write(0x0102, val);
    }

    pub fn arrange_for_abs_x(cpu: &mut RP2A03, val: u8) {
        // ins $0401, note the order.
        cpu.memory.write(1, 0x01);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0402, val);

        cpu.registers.x = 0x01;
    }

    pub fn arrange_for_abs_x_with_page_crossing(cpu: &mut RP2A03, val: u8) {
        // ins $04ff, note the order.
        cpu.memory.write(1, 0xff);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0500, val);

        cpu.registers.x = 0x01;
    }

    pub fn arrange_for_abs_y(cpu: &mut RP2A03, val: u8) {
        // ins $0401, note the order.
        cpu.memory.write(1, 0x01);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0402, val);

        cpu.registers.y = 0x01;
    }

    pub fn arrange_for_abs_y_with_page_crossing(cpu: &mut RP2A03, val: u8) {
        // ins $04ff, note the order.
        cpu.memory.write(1, 0xff);
        cpu.memory.write(2, 0x04);
        cpu.memory.write(0x0500, val);

        cpu.registers.y = 0x01;
    }

    pub fn arrange_for_indirect_x(cpu: &mut RP2A03, val: u8) {
        // ins $1234, note the order.
        cpu.memory.write(1, 0x90);
        cpu.memory.write(0x0091, 0x34);
        cpu.memory.write(0x0092, 0x12);
        cpu.memory.write(0x1234, val);

        cpu.registers.x = 0x01;
    }

    pub fn arrange_for_indirect_y(cpu: &mut RP2A03, val: u8) {
        // ins $1234, note the order.
        cpu.memory.write(1, 0x90);
        cpu.memory.write(0x0090, 0x33);
        cpu.memory.write(0x0091, 0x12);
        cpu.memory.write(0x1234, val);

        cpu.registers.y = 0x01;
    }

    pub fn arrange_for_indirect_y_with_page_crossing(cpu: &mut RP2A03, val: u8) {
        // ins $1234, note the order.
        cpu.memory.write(1, 0x90);
        cpu.memory.write(0x0090, 0xff);
        cpu.memory.write(0x0091, 0x12);
        cpu.memory.write(0x1300, val);

        cpu.registers.y = 0x01;
    }

    fn dumb_lda(registers: &mut Registers, val: u8) {
        registers.a = val;
    }

    #[test]
    fn opcode_fn_with_mode_imm() {
        opcode_fn_with_mode!(imm -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_imm(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_zero_page() {
        opcode_fn_with_mode!(zero_page -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_zero_page(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_zero_page_x() {
        opcode_fn_with_mode!(zero_page_x -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_zero_page_x(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_zero_page_y() {
        opcode_fn_with_mode!(zero_page_y -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_zero_page_y(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_rel() {
        opcode_fn_with_mode!(rel -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_rel(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_abs() {
        opcode_fn_with_mode!(abs -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_abs(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_abs_x() {
        opcode_fn_with_mode!(abs_x -> (target_fn, dumb_lda,
                                       page_crossed Cycle(0), or_else Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_abs_x(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_abs_y() {
        opcode_fn_with_mode!(abs_y -> (target_fn, dumb_lda,
                                       page_crossed Cycle(0), or_else Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_abs_y(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_indirect_x() {
        opcode_fn_with_mode!(indirect_x -> (target_fn, dumb_lda, Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_indirect_x(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }

    #[test]
    fn opcode_fn_with_mode_indirect_y() {
        opcode_fn_with_mode!(indirect_y -> (target_fn, dumb_lda,
                                            page_crossed Cycle(0), or_else Cycle(0)));

        let expected_val = 0x42;
        let mut cpu = {
            let mut c = RP2A03::new();
            arrange_for_indirect_y(&mut c, expected_val);

            c
        };

        target_fn(&mut cpu.registers, &mut cpu.memory);

        assert_eq!(cpu.registers.a, expected_val);
    }
}
