use crate::hardware::Machine;

enum Instructions {
    ExecuteMachienRoutine(u16), //0NNN
    ClearScreen, //00E0
    Jump(u16), //1NNN
    CallSubroutine(u16),//2NNN
ReturnSubroutine(),//00EE

    SetRegister(u8, u8),
    Add(u8, u8),
    SetIndex(u16),
    Display(u8, u8, u8),
}

fn execute_inst(machine: &mut Machine, instruction: Instructions) -> &Machine {
    match instruction {
        Instructions::ExecuteMachienRoutine(adress) => todo!("0NNN instruction is not supported"),
        Instructions::ClearScreen => {
            machine.clear_screen();
            machine.registers.inc_pc(1);
            machine
        }
        Instructions::Jump(jump_target) => {
            machine.registers.set_pc(jump_target);
            machine
        }
Instructions::CallSubroutine(subroutine_adress ) => {

            machine.registers.set_pc(subroutine_adress);
            machine


}

        _ => todo!("Not yet implemented"),
    }
}

mod tests_instructions {
    use super::*;
    use crate::registers::Registers;

    #[test]
    fn clear_screen_test() {
        let mut machine = Machine::new();
        machine.registers.set_pc(10);
        machine.display.display_buffer = vec![true; 64 * 32];
        execute_inst(&mut machine, Instructions::ClearScreen);

        assert_eq!(machine.display.display_buffer, vec![false; 64 * 32]);
        assert_eq!(machine.registers.pc, 11);
    }

    #[test]
    fn jump_test() {
        let mut machine = Machine::new();
        machine.registers.set_pc(10);

        execute_inst(&mut machine, Instructions::Jump(15));
        assert_eq!(machine.registers.pc, 15);
        
        execute_inst(&mut machine, Instructions::Jump(20));
        assert_eq!(machine.registers.pc, 20);
    }

    #[test]
    fn set_register_test() {
        let register = Registers::new();
        assert_eq!(register.v0, 0);
    }
}
