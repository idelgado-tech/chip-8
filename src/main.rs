use display::{DISPLAY_X_SIZE, DISPLAY_Y_SIZE};
use hardware::Machine;
use instructions::Instruction;

mod display;
mod hardware;
mod instructions;
mod ram;
mod registers;
mod rom;

pub fn decode_all_instructions(machine: &mut Machine) -> Vec<instructions::Instruction> {
    let mut instruction = machine.fetch_instruction();
    let mut instructions_vector: Vec<instructions::Instruction> = Vec::new();

    while instruction != 0 {
        let instruction_decoded = instructions::Instruction::decode_instruction(instruction);
        println!("here isntruction {:?}", instruction_decoded);
        instructions_vector.push(instruction_decoded);
        instruction = machine.fetch_instruction();
    }
    instructions_vector
}

fn main() {
    // let mut machine = Machine::new();

    // machine.display.set_pixel(0, 0, true);
    // machine.display.set_pixel(1, 1, true);

    // machine.display.set_pixel(15, 25, true);
    // machine.display.set_pixel(16, 25, true);
    // machine.display.set_pixel(17, 25, true);
    // machine.display.set_pixel(18, 25, true);
    // machine.display.set_pixel(19, 25, true);
    // machine.display.set_pixel(15, 26, true);
    // machine.display.set_pixel(14, 26, true);

    // machine
    //     .display
    //     .set_pixel((DISPLAY_X_SIZE - 1) as u8, (DISPLAY_Y_SIZE - 1) as u8, true);
    // machine
    //     .display
    //     .set_pixel((DISPLAY_X_SIZE - 2) as u8, (DISPLAY_Y_SIZE - 2) as u8, true);

    // machine.display.print();

    // let rom = rom::get_file_as_byte_vec(&"resources/IBM Logo.ch8".to_string());
    // let rom = rom::get_file_as_byte_vec(&"resources/random_number_test.ch8".to_string());
    let rom = rom::get_file_as_byte_vec(&"resources/test_opcode.ch8".to_string());

    println!("START ROM");
    rom::print_rom(&rom);
    println!("END ROM\n");

    let mut machine = Machine::new();
    rom::setup_rom_to_memory(&mut machine, &rom);

    machine.instruction_loop();
    // println!("START DECODE");    
    // println!("{:?}", decode_all_instructions(&mut machine));
    // println!("END DECODE\n");
}
