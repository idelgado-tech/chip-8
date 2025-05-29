pub const DISPLAY_X_SIZE: usize = 64;
pub const DISPLAY_Y_SIZE: usize = 32;

pub struct MachineDisplay {
    pub display_buffer: Vec<bool>,
    pub v_ram_changed :bool
}

pub fn pos_from_index(index: usize) -> (usize, usize) {
    let y = index / DISPLAY_X_SIZE;
    let x = index % DISPLAY_X_SIZE;
    (x, y)
}

pub fn index_from_pos(x: usize, y: usize) -> usize {
    (y * DISPLAY_X_SIZE) + x
}

impl MachineDisplay {
    pub fn new() -> MachineDisplay {
        MachineDisplay {
            display_buffer: vec![false; 64 * 32],
            v_ram_changed :false
        }
    }

    pub fn clear(&mut self) {
        self.display_buffer = vec![false; 64 * 32];
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        if x >= DISPLAY_X_SIZE as u8 || y >= DISPLAY_Y_SIZE as u8 {
            panic!("Position out of bound : x {} , y {}",x,y)
        }
        self.display_buffer[index_from_pos(x as usize, y as usize)]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        if x >= DISPLAY_X_SIZE as u8 || y >= DISPLAY_Y_SIZE as u8 {
            panic!("Position out of bound")
        }
        self.display_buffer[index_from_pos(x as usize, y as usize)] = value;
    }

    pub fn xor_pixel(&mut self, x: u8, y: u8, value: bool) {
        if x >= DISPLAY_X_SIZE as u8 || y >= DISPLAY_Y_SIZE as u8 {
            panic!("Position out of bound")
        }
        self.display_buffer[index_from_pos(x as usize, y as usize)] ^= value;
    }

    pub fn print(&self) {
        println!("==========================================");
        for i in 0..DISPLAY_Y_SIZE {
            for j in 0..DISPLAY_X_SIZE {
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
