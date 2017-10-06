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

#[inline]
pub fn compose_addr(addr_high: u8, addr_low: u8) -> u16 {
    ((addr_high as u16) << 8) + addr_low as u16
}

#[inline]
pub fn compose_indexed_addr(addr_high: u8, addr_low: u8, index: u8) -> (u16, bool) {
    let (addr, page_crossed) = match addr_low.overflowing_add(index) {
        (addr_low, true) => {
            let (addr_high, _overflowed) = addr_high.overflowing_add(1);

            (compose_addr(addr_high, addr_low), true)
        }
        (addr_low, false) => (compose_addr(addr_high, addr_low), false),
    };

    (addr, page_crossed)
}

#[cfg(test)]
pub mod test {
    use cpu::RP2A03;

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
}
