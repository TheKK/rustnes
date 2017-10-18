use opcode::OpCode;
use opcode::Cycle;

const MEM_ADDR_MAX: usize = 0xffff;

#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    p: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: 0b00100000, // The unused bit should always be logical one.
            pc: 0x00,
            sp: 0x00,
        }
    }
}

macro_rules! bit_flag_getter_setter {
    ($setter_name: ident, $getter_name: ident, $bit_no: expr) => {
        #[inline]
        pub fn $setter_name(&mut self, flag: bool) {
            if flag {
                self.p |= 1 << $bit_no;
            } else {
                self.p &= !(1 << $bit_no);
            }
        }

        #[inline]
        pub fn $getter_name(&self) -> bool {
            ((self.p >> $bit_no) & 1) == 1
        }
    }
}

impl Registers {
    #[inline]
    pub fn p(&self) -> u8 {
        self.p
    }

    bit_flag_getter_setter!(set_carry_flag, carry_flag, 0);
    bit_flag_getter_setter!(set_zero_flag, zero_flag, 1);
    bit_flag_getter_setter!(set_interrupt_disable_flag, interrupt_disable_flag, 2);
    bit_flag_getter_setter!(set_decimal_mode_flag, decimal_mode_flag, 3);
    bit_flag_getter_setter!(set_break_command_flag, break_command_flag, 4);
    bit_flag_getter_setter!(set_overflow_flag, overflow_flag, 6);
    bit_flag_getter_setter!(set_sign_flag, sign_flag, 7);
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
    pub memory: Memory,
    pub registers: Registers,
    current_cycles: u32,
}

impl RP2A03 {
    pub fn new() -> RP2A03 {
        RP2A03 {
            memory: Memory::new(),
            registers: Registers::new(),
            current_cycles: 0,
        }
    }

    pub fn execute(&mut self) -> Cycle {
        let pc = self.registers.pc;

        let opcode = OpCode::from(self.memory.read(pc as u16));
        let opcode_fn = opcode.get_fn();

        let Cycle(cycles_num) = opcode_fn(&mut self.registers, &mut self.memory);

        self.registers.pc += (1 + opcode.operands_num()) as u16;
        self.current_cycles += cycles_num;

        Cycle(cycles_num)
    }
}
