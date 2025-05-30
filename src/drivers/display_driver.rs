use std::ops::Mul;

use minifb::{Window, WindowOptions};

use crate::{
    display::{self, VRAM_WIDTH},
    hardware::Machine,
    utils::{self, index_from_pos, pos_from_index},
};

pub const PIXEL_SIZE: usize = 15;

pub const VBUFFER_WIDTH: usize = display::VRAM_WIDTH * PIXEL_SIZE;
pub const VBUFFER_HEIGHT: usize = display::VRAM_HEIGTH * PIXEL_SIZE;

pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub const AZURE_BLUE: u32 = from_u8_rgb(
    (0.0 * 255.0) as u8,
    (0.5 * 255.0) as u8,
    (255.0 * 1.0) as u8,
);

pub const LIGHT_VIOLET: u32 = from_u8_rgb(
    (0.5 * 255.0) as u8,
    (0.5 * 255.0) as u8,
    (255.0 * 1.0) as u8,
);

pub const BLACK: u32 = from_u8_rgb(
    (0.0 * 255.0) as u8,
    (0.0 * 255.0) as u8,
    (0.0 * 255.0) as u8,
);

pub fn new_window() -> Window {
    Window::new(
        "Test - ESC to exit",
        VBUFFER_WIDTH,
        VBUFFER_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
}

fn vbuffer_from_vram(display_buffer: &Vec<bool>) -> Vec<u32> {
    let mut vbuffer: Vec<u32> =
        vec![LIGHT_VIOLET; VBUFFER_HEIGHT.mul(VBUFFER_WIDTH).try_into().unwrap()];
    for (index, pixel) in display_buffer.iter().enumerate() {
        // println!( "here pixel et index {} {} ", &pixel,index);
        if *pixel {
            let (x, y) = pos_from_index(index, VRAM_WIDTH);
            for i in 0..PIXEL_SIZE {
                for j in 0..PIXEL_SIZE {
                    vbuffer[index_from_pos(x * PIXEL_SIZE + i, y * PIXEL_SIZE + j, VBUFFER_WIDTH)] = BLACK;
                }
            }
        }
    }
    vbuffer
}

impl Machine {
    pub fn update_screen(&mut self) {
        let new_buffer = vbuffer_from_vram(&self.display.display_buffer);
        self.window
            .update_with_buffer(&new_buffer, VBUFFER_WIDTH, VBUFFER_HEIGHT)
            .unwrap();
        self.display.v_ram_changed = false;
    }
}
