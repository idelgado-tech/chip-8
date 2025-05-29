use crate::display::MachineDisplay;
use crate::instructions;
use crate::ram::Ram;
use crate::registers::Registers;

pub struct Machine {
    pub display: MachineDisplay,
    pub ram: Ram,
    pub registers: Registers,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            display: MachineDisplay::new(),
            ram: Ram::new(),
            registers: Registers::new(),
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
                self.display.print();
                self.display.v_ram_changed = false;
            }
        }
    }
}
