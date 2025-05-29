use std::ops::Shl;

use rand::Rng;

use crate::{
    display::{self, DISPLAY_X_SIZE},
    hardware::Machine,
};
const INSTRUCTION_PAR_SEC: usize = 700;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ExecuteMachienRoutine(u16),       //0NNN
    ClearScreen,                      //00E0
    Jump(u16),                        //1NNN
    JumpOffset(u8, u16),              //BXNN OR BNNN
    CallSubroutine(u16),              //2NNN
    ReturnSubroutine,                 //00EE
    SetRegister(u8, u8),              //6XNN
    AddToRegister(u8, u8),            //7XNN
    SetRegisterFrom(u8, u8),          //8XY0
    BinaryOR(u8, u8),                 //8XY1
    BinaryAND(u8, u8),                //8XY2
    BinaryXOR(u8, u8),                //8XY3
    RegisterAddition(u8, u8),         //8XY4
    SubstractXY(u8, u8),              //8XY5
    ShiftLeft(u8, u8),                //8XY6
    SubstractYX(u8, u8),              //8XY7
    ShiftRight(u8, u8),               //8XYE
    SetIndex(u16),                    //ANNN
    SkipIfEqual(u8, u8),              //3XNN
    SkipIfNotEqual(u8, u8),           //4XNN
    SkipIfRegistersEquals(u8, u8),    //5XY0
    SkipIfRegistersNotEquals(u8, u8), //9XY0
    Random(u8, u8),                   //CXNN
    SetRegisterToDelay(u8),           //FX07
    SetDelayTimer(u8),                //FX15
    SetSoundTimer(u8),                //FX18
    AddToIndex(u8),                   //FX1E
    GetKey(u8),                       //FX0A
    SkipIfKey(u8),                    //EX9E
    SkipIfNotKey(u8),                 //EXA1
    SetIndexTofont(u8),               //FX29
    DecimalConversion(u8),            //FX33
    StoreMemory(u8),                  //FX55
    LoadMemory(u8),                   //FX65
    Display {
        //DXYN
        register_x: u8, //X
        register_y: u8, //Y
        rows: u8,       //N
    },
}

impl Instruction {
    fn decode_0_instruction(code: u16) -> Instruction {
        match code {
            0x00E0 => Self::ClearScreen,
            0x00EE => Self::ReturnSubroutine,
            _ => Self::ExecuteMachienRoutine(extract_last_3_hex_digits(code)), //0NNN
        }
    }

    fn decode_1_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::Jump(extract_last_3_hex_digits(code)), //1NNN
        }
    }

    fn decode_2_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::CallSubroutine(extract_last_3_hex_digits(code)), //2NNN
        }
    }

    fn decode_3_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SkipIfEqual(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //3XNN
        }
    }

    fn decode_4_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SkipIfNotEqual(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //4XNN
        }
    }

    fn decode_5_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SkipIfRegistersEquals(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //5XY0
        }
    }

    fn decode_6_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SetRegister(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //6XNN
        }
    }

    fn decode_7_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::AddToRegister(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //7XNN
        }
    }

    fn decode_8_instruction(code: u16) -> Instruction {
        match extract_last_hex_digit(code) {
            0 => Self::SetRegisterFrom(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY0
            1 => Self::BinaryOR(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY1
            2 => Self::BinaryAND(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY2
            3 => Self::BinaryXOR(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY3
            4 => Self::RegisterAddition(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY4
            5 => Self::SubstractXY(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY5
            7 => Self::SubstractYX(
                extract_second_hex_digit(code),
                extract_third_hex_digit(code),
            ), //8XY7
            _ => todo!("not decodable"),
        }
    }

    fn decode_9_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SkipIfRegistersNotEquals(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //9XY0
        }
    }

    fn decode_a_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::SetIndex(extract_last_3_hex_digits(code)), //ANNN
        }
    }

    fn decode_b_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::JumpOffset(
                extract_second_hex_digit(code),
                extract_last_3_hex_digits(code),
            ), //BXNN or BNNN
        }
    }

    fn decode_c_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::Random(
                extract_second_hex_digit(code),
                extract_last_2_hex_digits(code),
            ), //CXNN
        }
    }

    fn decode_d_instruction(code: u16) -> Instruction {
        match code {
            _ => Self::Display {
                register_x: extract_second_hex_digit(code),
                register_y: extract_third_hex_digit(code),
                rows: extract_last_hex_digit(code),
            }, //DXYN
        }
    }

    fn decode_e_instruction(code: u16) -> Instruction {
        match extract_last_2_hex_digits(code) {
            0x9E => Self::SkipIfKey(extract_second_hex_digit(code)), //EX9E
            0xA1 => Self::SkipIfNotKey(extract_second_hex_digit(code)), //EXA1
            _ => todo!("INSTRUCTION NOT yet decoded : {:#x}", code),
        }
    }

    fn decode_f_instruction(code: u16) -> Instruction {
        match extract_last_2_hex_digits(code) {
            0x07 => Self::SetRegisterToDelay(extract_second_hex_digit(code)),
            0x15 => Self::SetDelayTimer(extract_second_hex_digit(code)),
            0x18 => Self::SetSoundTimer(extract_second_hex_digit(code)),
            0x1E => Self::AddToIndex(extract_second_hex_digit(code)),
            0x0A => Self::GetKey(extract_second_hex_digit(code)),
            0x29 => Self::SetIndexTofont(extract_second_hex_digit(code)),
            0x33 => Self::DecimalConversion(extract_second_hex_digit(code)),
            0x55 => Self::StoreMemory(extract_second_hex_digit(code)),
            0x65 => Self::LoadMemory(extract_second_hex_digit(code)),
            _ => todo!("INSTRUCTION NOT yet decoded : {:#x}", code), //todo Better message
        }
    }

    pub fn decode_instruction(code: u16) -> Instruction {
        match extract_first_hex_digit(code) {
            0x0 => Self::decode_0_instruction(code),
            0x1 => Self::decode_1_instruction(code),
            0x2 => Self::decode_2_instruction(code),
            0x3 => Self::decode_3_instruction(code),
            0x4 => Self::decode_4_instruction(code),
            0x5 => Self::decode_5_instruction(code),
            0x6 => Self::decode_6_instruction(code),
            0x7 => Self::decode_7_instruction(code),
            0x8 => Self::decode_8_instruction(code),
            0x9 => Self::decode_9_instruction(code),
            0xA => Self::decode_a_instruction(code),
            0xB => Self::decode_b_instruction(code),
            0xC => Self::decode_c_instruction(code),
            0xD => Self::decode_d_instruction(code),
            0xE => Self::decode_e_instruction(code),
            0xF => Self::decode_f_instruction(code),
            _ => todo!("INSTRUCTION NOT yet decoded : {:#x}", code),
        }
    }
}

pub fn execute_inst(machine: &mut Machine, instruction: Instruction) {
    match instruction {
        Instruction::ExecuteMachienRoutine(adress) => todo!("0NNN instruction is not supported"),
        Instruction::ClearScreen => {
            machine.clear_screen();
        }
        Instruction::Jump(jump_target) => {
            machine.registers.set_pc(jump_target);
        }
        Instruction::JumpOffset(_, nnn) => {
            // Implementing BNNN configuere BXNN ?
            machine
                .registers
                .set_pc(nnn + machine.registers.v_registers[0] as u16);
        }
        Instruction::CallSubroutine(subroutine_adress) => {
            machine.registers.set_pc(subroutine_adress);
        }
        Instruction::SetIndex(value) => {
            machine.registers.ir = value;
        }
        Instruction::SetRegister(x, value) => {
            machine.registers.v_registers[x as usize] = value;
        }
        Instruction::AddToRegister(x, value) => {
            machine.registers.v_registers[x as usize] =
                machine.registers.v_registers[x as usize].wrapping_add(value);
        }
        Instruction::SkipIfEqual(x, value) => {
            if machine.registers.v_registers[x as usize] == value {
                machine.registers.inc_pc(2);
            }
        }
        Instruction::SkipIfNotEqual(x, value) => {
            if machine.registers.v_registers[x as usize] != value {
                machine.registers.inc_pc(2);
            }
        }
        Instruction::SkipIfRegistersEquals(x, y) => {
            if machine.registers.v_registers[x as usize]
                == machine.registers.v_registers[y as usize]
            {
                machine.registers.inc_pc(2);
            }
        }
        Instruction::SkipIfRegistersNotEquals(x, y) => {
            if machine.registers.v_registers[x as usize]
                != machine.registers.v_registers[y as usize]
            {
                machine.registers.inc_pc(2);
            }
        }
        Instruction::SetRegisterFrom(x, y) => {
            machine.registers.v_registers[x as usize] = machine.registers.v_registers[y as usize];
        }
        Instruction::BinaryOR(x, y) => {
            machine.registers.v_registers[x as usize] = machine.registers.v_registers[x as usize]
                | machine.registers.v_registers[y as usize];
        }
        Instruction::BinaryAND(x, y) => {
            machine.registers.v_registers[x as usize] = machine.registers.v_registers[x as usize]
                & machine.registers.v_registers[y as usize];
        }
        Instruction::BinaryXOR(x, y) => {
            machine.registers.v_registers[x as usize] = machine.registers.v_registers[x as usize]
                ^ machine.registers.v_registers[y as usize];
        }
        Instruction::RegisterAddition(x, y) => {
            let (result, overflow) = machine.registers.v_registers[x as usize]
                .overflowing_add(machine.registers.v_registers[y as usize]);
            machine.registers.v_registers[0xf] = overflow as u8;
            machine.registers.v_registers[x as usize] = result;
        }
        Instruction::SubstractXY(x, y) => {
            let (result, overflow) = machine.registers.v_registers[x as usize]
                .overflowing_sub(machine.registers.v_registers[y as usize]);
            machine.registers.v_registers[0xf] = overflow as u8;
            machine.registers.v_registers[x as usize] = result;
        }
        Instruction::SubstractYX(x, y) => {
            let (result, overflow) = machine.registers.v_registers[y as usize]
                .overflowing_sub(machine.registers.v_registers[x as usize]);
            machine.registers.v_registers[0xf] = overflow as u8;
            machine.registers.v_registers[x as usize] = result;
        }
        Instruction::ShiftLeft(x, y) => {
            machine.registers.v_registers[0xf] =
                machine.registers.v_registers[x as usize] & 0x80 >> 7;
            machine.registers.v_registers[x as usize] =
                machine.registers.v_registers[x as usize] << 1;
        } // prevoir istructions differentes pour COSMAC et SUPER-CHIP
        Instruction::ShiftRight(x, y) => {
            machine.registers.v_registers[0xf] = machine.registers.v_registers[x as usize] & 0x01;
            machine.registers.v_registers[x as usize] =
                machine.registers.v_registers[x as usize] >> 1;
        } // prevoir istructions differentes pour COSMAC et SUPER-CHIP
        Instruction::Random(x, nn) => {
            let number: u8 = rand::rng().random();
            machine.registers.v_registers[x as usize] = number & nn;
        }
        Instruction::SetRegisterToDelay(x) => {
            machine.registers.v_registers[x as usize] = machine.registers.delay_timer;
        }
        Instruction::SetDelayTimer(x) => {
            machine.registers.delay_timer = machine.registers.v_registers[x as usize];
        }
        Instruction::SetSoundTimer(x) => {
            machine.registers.sound_timer = machine.registers.v_registers[x as usize];
        }
        Instruction::DecimalConversion(x) => {
            let number = machine.registers.v_registers[x as usize];
            let index = machine.registers.ir as usize;
            machine.ram.ram[index] = number / 100;
            machine.ram.ram[index + 1] = number % 100 / 10;
            machine.ram.ram[index + 2] = number % 10;
        }
        // todo configuration old rules
        Instruction::LoadMemory(x) => {
            for register in 0..x as usize {
                machine.ram.ram[machine.registers.ir as usize + register] =
                    machine.registers.v_registers[register]
            }
        }
        Instruction::StoreMemory(x) => {
            for register in 0..x as usize {
                machine.registers.v_registers[register] =
                    machine.ram.ram[machine.registers.ir as usize + register];
            }
        }
        Instruction::SetIndexTofont(x) => {
            let char = machine.registers.v_registers[x as usize] as u16;
            if char > 15 {
                panic!(
                    "in Instruction SetIndexTofont X [x={}]should not go over 15",
                    x
                );
            }
            let index_value = 0x50 + 5 * char;
            machine.registers.ir = index_value;
        }
        Instruction::Display {
            rows,
            register_x,
            register_y,
        } => {
            machine.registers.v_registers[0xf] = 0;

            for byte in 0..rows as u8 {
                let y = (machine.registers.v_registers[register_y as usize] + byte)
                    % display::DISPLAY_Y_SIZE as u8;

                for bit in 0..8 {
                    let x = (machine.registers.v_registers[register_x as usize] + bit)
                        % display::DISPLAY_X_SIZE as u8;

                    let color =
                        (machine.ram.read(machine.registers.ir + byte as u16) >> (7 - bit)) & 1;

                    machine.registers.v_registers[0x0f] |=
                        color & machine.display.get_pixel(x, y) as u8;
                    machine.display.xor_pixel(x, y, color != 0);
                }
            }
            machine.display.v_ram_changed = true;
        }

        _ => todo!("Instruction not yet implemented : {:?} ", instruction),
    }
}

pub fn extract_last_hex_digit(hex: u16) -> u8 {
    (hex & 0x00F) as u8
}
pub fn extract_last_2_hex_digits(hex: u16) -> u8 {
    (hex & 0x00FF) as u8
}

pub fn extract_last_3_hex_digits(hex: u16) -> u16 {
    (hex & 0x0FFF) as u16
}

pub fn extract_second_hex_digit(hex: u16) -> u8 {
    ((hex & 0x0F00) >> 4 * 2) as u8
}

pub fn extract_first_hex_digit(hex: u16) -> u8 {
    ((hex & 0xF000) >> 4 * 3) as u8
}

pub fn extract_third_hex_digit(hex: u16) -> u8 {
    ((hex & 0x00F0) >> 4 * 1) as u8
}

mod tests_instructions {
    use super::*;
    use crate::registers::Registers;

    #[test]
    fn extract_third_digit_test() {
        assert_eq!(extract_third_hex_digit(0xFFFF), 0xF);
        assert_eq!(extract_third_hex_digit(0x00AB), 0xA);
        assert_eq!(extract_third_hex_digit(0x1CB6), 0xB);
    }

    #[test]
    fn extract_second_digit_test() {
        assert_eq!(extract_second_hex_digit(0xFFFF), 0xF);
        assert_eq!(extract_second_hex_digit(0x00AB), 0x0);
        assert_eq!(extract_second_hex_digit(0x1CB6), 0xC);
    }

    #[test]
    fn extract_first_digit_test() {
        assert_eq!(extract_first_hex_digit(0xFFFF), 0xF);
        assert_eq!(extract_first_hex_digit(0x00AB), 0x0);
        assert_eq!(extract_first_hex_digit(0x1CB6), 0x1);
    }

    #[test]
    fn extract_last_digit_test() {
        assert_eq!(extract_last_hex_digit(0xFFFF), 0xF);
        assert_eq!(extract_last_hex_digit(0x00AB), 0xB);
        assert_eq!(extract_last_hex_digit(0x1CB6), 0x6);
    }

    #[test]
    fn extract_last_2_digits_test() {
        assert_eq!(extract_last_2_hex_digits(0xFFFF), 0xFF);
        assert_eq!(extract_last_2_hex_digits(0x00AB), 0xAB);
        assert_eq!(extract_last_2_hex_digits(0x1CB6), 0xB6);
    }

    #[test]
    fn extract_last_3_digits_test() {
        assert_eq!(extract_last_3_hex_digits(0xFFFF), 0xFFF);
        assert_eq!(extract_last_3_hex_digits(0x00AB), 0xAB);
        assert_eq!(extract_last_3_hex_digits(0x1CB6), 0xCB6);
    }

    #[test]
    fn clear_screen_test() {
        let mut machine = Machine::new();
        machine.display.display_buffer = vec![true; 64 * 32];
        execute_inst(&mut machine, Instruction::ClearScreen);

        assert_eq!(machine.display.display_buffer, vec![false; 64 * 32]);
    }

    #[test]
    fn decimal_conversion_test() {
        let mut machine = Machine::new();
        machine.registers.ir = 0xFF;
        let number = 156;
        machine.registers.v_registers[0x00] = number;
        execute_inst(&mut machine, Instruction::DecimalConversion(0x00));
        assert_eq!(machine.ram.ram[0xFF], 1);
        assert_eq!(machine.ram.ram[0xFF + 1], 5);
        assert_eq!(machine.ram.ram[0xFF + 2], 6);

        let number_2 = 078;
        machine.registers.v_registers[0x01] = number_2;
        execute_inst(&mut machine, Instruction::DecimalConversion(0x01));
        assert_eq!(machine.ram.ram[0xFF], 0);
        assert_eq!(machine.ram.ram[0xFF + 1], 7);
        assert_eq!(machine.ram.ram[0xFF + 2], 8);
    }

    #[test]
    fn jump_test() {
        let mut machine = Machine::new();
        machine.registers.set_pc(10);

        execute_inst(&mut machine, Instruction::Jump(15));
        assert_eq!(machine.registers.pc, 15);

        execute_inst(&mut machine, Instruction::Jump(20));
        assert_eq!(machine.registers.pc, 20);
    }

    #[test]
    fn set_register_test() {
        let register = Registers::new();
        assert_eq!(register.v_registers[0x0], 0);
    }

    #[test]
    fn decode_d_instruction_test() {
        assert_eq!(
            Instruction::decode_instruction(0xd01f),
            Instruction::Display {
                register_x: 0,
                register_y: 1,
                rows: 0xf
            }
        )
    }
}
