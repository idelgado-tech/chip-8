use std::thread;
use std::time::Duration;

use minifb::Window;

use crate::display::MachineDisplay;
use crate::drivers::display_driver;
use crate::instructions;
use crate::ram::Ram;
use crate::registers::Registers;

pub const SLEEP_DURATION :Duration = Duration::from_millis(20);

pub struct Machine {
    pub display: MachineDisplay,
    pub ram: Ram,
    pub registers: Registers,
    pub window: Window,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            display: MachineDisplay::new(),
            ram: Ram::new(),
            registers: Registers::new(),
            window: display_driver::new_window(),
        }
    }

    pub fn clear_screen(&mut self) -> () {
        self.display.clear();
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let premier_partie = self.ram.read(self.registers.pc);
        let seconde_partie = self.ram.read(self.registers.pc + 1);
        self.registers.inc_pc(2);
        ((premier_partie as u16) << 8) | seconde_partie as u16
    }

    pub fn instruction_loop(&mut self) {
        loop {
            let instruction = self.fetch_instruction();
            let decoded_instruction = instructions::Instruction::decode_instruction(instruction);
            instructions::execute_inst(self, decoded_instruction);
            if self.display.v_ram_changed {
                self.update_screen();
            }
            thread::sleep(SLEEP_DURATION);
        }//TODO put in a bigger loop for key and sound gestion
    }
}
