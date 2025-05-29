use std::fs::{self, File};
use std::io::Read;

use crate::hardware::Machine;

pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn setup_rom_to_memory(machine: &mut Machine, rom: &Vec<u8>) {
    let first_pos: u16 = 0x200;
    for i in 0..rom.len() {
        machine.ram.write(first_pos + i as u16, rom[i]);
        machine.registers.set_pc(first_pos);
    }
}

pub fn print_rom(rom: &Vec<u8>) {
    let mut i = 0;
    while i < rom.len() && i + 1 < rom.len() {
        let premier_partie = rom[i];
        let seconde_partie = rom[i + 1];
        i += 2;
        println!(
            "{:#06x}",
            ((premier_partie as u16) << 8) | seconde_partie as u16
        )
    }
}
