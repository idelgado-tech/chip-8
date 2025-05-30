use core::time;
use std::thread::sleep;

use display::{VRAM_HEIGTH, VRAM_WIDTH};
use drivers::display_driver::{self, from_u8_rgb, VBUFFER_HEIGHT, VBUFFER_WIDTH};
use hardware::Machine;
use instructions::Instruction;
use minifb::Key;

mod display;
mod drivers;
mod hardware;
mod instructions;
mod ram;
mod registers;
mod rom;
mod utils;

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

    //    machine
    //     .display
    //     .set_pixel((VRAM_WIDTH - 1) as u8, (VRAM_HEIGTH - 1) as u8, true);

    // machine
    //     .display
    //     .set_pixel((VRAM_WIDTH - 2) as u8, (VRAM_HEIGTH - 2) as u8, true);

    // machine.display.print();

    // let rom = rom::get_file_as_byte_vec(&"resources/IBM Logo.ch8".to_string());
    // let rom = rom::get_file_as_byte_vec(&"resources/random_number_test.ch8".to_string());
    let rom = rom::get_file_as_byte_vec(&"resources/test_opcode.ch8".to_string());

    // println!("START ROM");
    // rom::print_rom(&rom);
    // println!("END ROM\n");

    let mut machine = Machine::new();
    rom::setup_rom_to_memory(&mut machine, &rom);

    // println!("START DECODE");
    // println!("{:?}", decode_all_instructions(&mut machine));
    // println!("END DECODE\n");


    // Limit to max ~60 fps update rate
    machine.update_screen();
    machine.window.set_target_fps(60);
    while machine.window.is_open() && !machine.window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        machine.instruction_loop();
    }
}
