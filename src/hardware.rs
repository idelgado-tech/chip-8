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

pub struct Registers {
    pub pc: u16,
    pub ir: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub v0: u16,
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub v4: u16,
    pub v5: u16,
    pub v6: u16,
    pub v7: u16,
    pub v8: u16,
    pub v9: u16,
    pub va: u16,
    pub vb: u16,
    pub vc: u16,
    pub vd: u16,
    pub ve: u16,
    pub vf: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            ir: 0,
            stack: 0,
            delay_timer: 0,
            sound_timer: 0,

            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }
}

pub const DISPLAY_X: usize = 64;
pub const DISPLAY_Y: usize = 32;

pub struct Display {
    pub display_buffer: Vec<bool>,
}
impl Display {
    pub fn new() -> Display {
        Display {
            display_buffer: vec![false; 64 * 32],
        }
    }
}
