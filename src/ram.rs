use crate::display::Display;

pub const RAM_SIZE: usize = 4 * 1024;

pub struct Ram {
    pub ram: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            ram: vec![0; RAM_SIZE],
        }
    }
}
