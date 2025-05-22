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

    pub fn clear (& mut self) -> & Display {
        self.display_buffer =  vec![false; 64 * 32];
        self
    }
}   