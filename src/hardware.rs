use crate::display::Display;
use crate::ram::Ram;
use crate::registers::Registers;

pub struct Machine {
    pub display: Display,
    pub ram: Ram,
    pub registers: Registers,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            display: Display::new(),
            ram: Ram::new(),
            registers: Registers::new(),
        }
    }

    pub fn clear_screen(&mut self) -> () {
        self.display.clear();
    }
}
