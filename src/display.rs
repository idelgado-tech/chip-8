use crate::utils::index_from_pos;

pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGTH: usize = 32;

pub struct MachineDisplay {
    pub display_buffer: Vec<bool>,
    pub v_ram_changed: bool,
}

impl MachineDisplay {
    pub fn new() -> MachineDisplay {
        MachineDisplay {
            display_buffer: vec![false; 64 * 32],
            v_ram_changed: false,
        }
    }

    pub fn clear(&mut self) {
        self.display_buffer = vec![false; 64 * 32];
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        if x >= VRAM_WIDTH as u8 || y >= VRAM_HEIGTH as u8 {
            panic!("Position out of bound : x {} , y {}", x, y)
        }
        self.display_buffer[index_from_pos(x as usize, y as usize, VRAM_WIDTH)]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        if x >= VRAM_WIDTH as u8 || y >= VRAM_HEIGTH as u8 {
            panic!("Position out of bound")
        }
        self.display_buffer[index_from_pos(x as usize, y as usize, VRAM_WIDTH)] = value;
    }

    pub fn xor_pixel(&mut self, x: u8, y: u8, value: bool) {
        if x >= VRAM_WIDTH as u8 || y >= VRAM_HEIGTH as u8 {
            panic!("Position out of bound")
        }
        self.display_buffer[index_from_pos(x as usize, y as usize, VRAM_WIDTH)] ^= value;
    }

    pub fn print(&self) {
        println!("==========================================");
        for i in 0..VRAM_HEIGTH {
            for j in 0..VRAM_WIDTH {
                let value = self.get_pixel(j as u8, i as u8);
                let char = match value {
                    true => "X",
                    false => " ",
                };
                print!("{}", char);
            }
            print!("\n")
        }
        println!("==========================================");
    }
}
