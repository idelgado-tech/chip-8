const STACK_LIMIT: usize = 16;

pub struct Registers {
    pub pc: u16,
    pub ir: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v_registers: [u8; 16],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            ir: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            v_registers: [0; 16],
        }
    }

    pub fn set_pc(&mut self, new_value: u16) -> () {
        self.pc = new_value
    }

    pub fn inc_pc(&mut self, inc_step: u16) -> () {
        self.pc += inc_step
    }

    pub fn push_stack(&mut self, value: u16) -> () {
        if self.stack.len() >= STACK_LIMIT {
            panic!("STACK OVERFLOW")
        }
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.stack.pop().unwrap_or_else(|| panic!("stack empty"))
    }
}

mod tests_instructions {
    use super::*;
    use crate::registers::Registers;

    #[test]
    fn new_register_test() {
        let register = Registers::new();
        assert_eq!(register.v_registers[0x0], 0);
    }
}
