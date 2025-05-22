pub struct Registers {
    pub pc: u16,
    pub ir: u16,
    pub stack: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub v0: u16,
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub v4: u16,
    pub v5: u16,
    pub v6: u16,
    pub v7: u16,
    pub v8: u16,
    pub v9: u16,
    pub va: u16,
    pub vb: u16,
    pub vc: u16,
    pub vd: u16,
    pub ve: u16,
    pub vf: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            ir: 0,
            stack: 0,
            delay_timer: 0,
            sound_timer: 0,

            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }

    pub fn set_pc(&mut self, new_value: u16) -> () {
        self.pc = new_value
    }

    pub fn inc_pc(&mut self, inc_step: u16) -> () {
        self.pc += inc_step
    }

    pub fn add_stack(&mut self, value: u16) -> (){
        self.stack = value;
    }

    pub fn pop_stack (&mut self) -> u16 {
        let value = self.stack;
        self.stack = 0;
        value //TODO implement true stack
    }


}

mod tests_instructions {
    use super::*;
    use crate::registers::Registers;

    #[test]
    fn set_register_test() {
        let register = Registers::new();
        assert_eq!(register.v0, 0);
    }
}
