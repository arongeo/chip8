// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// display.rs file
//

#[path = "config.rs"]
mod config;

use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;

pub struct Display {
    pub pixels: [[bool; config::CHIP8_DISPLAY_HEIGHT]; config::CHIP8_DISPLAY_WIDTH],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; config::CHIP8_DISPLAY_HEIGHT]; config::CHIP8_DISPLAY_WIDTH],
        }
    }

    fn check_pixel_in_bounds(x: usize, y: usize) {
        if ((x < config::CHIP8_DISPLAY_WIDTH) && (0 <= x)) == false {
            panic!("Error: Pixel out of bounds");
        }
        if ((y < config::CHIP8_DISPLAY_HEIGHT) && (0 <= y)) == false {
            panic!("Error: Pixel out of bounds");
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: bool) {
        Self::check_pixel_in_bounds(x, y);
        self.pixels[x][y] = val;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        Self::check_pixel_in_bounds(x, y);
        self.pixels[x][y]
    }
    
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        for i in 0..config::CHIP8_DISPLAY_WIDTH {
            for j in 0..config::CHIP8_DISPLAY_HEIGHT {
                if self.get_pixel(i, j) == true {                   
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new((i*10) as i32, (j*10) as i32, 10, 10));
                }
                if self.get_pixel(i, j) == false {                   
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new((i*10) as i32, (j*10) as i32, 10, 10));
                }
            }
        }
    }
    
    /// Clear screen
    pub fn clear(&mut self) {
        self.pixels = [[false; config::CHIP8_DISPLAY_HEIGHT]; config::CHIP8_DISPLAY_WIDTH];
    }

    /// XOR a byte onto the screen from coordinates x and y
    /// Return true if pixel collision happened.
    pub fn draw_byte(&mut self, x: usize, y: usize, byte: u8) -> bool {
        let bits = config::get_bit_values(byte);
        let mut collision_happened = false;
        for pixel in 0..8 {
            let curr_val = self.get_pixel((x + pixel) & 63, y & 31);
            self.set_pixel((x + pixel) & 63, y & 31, curr_val ^ bits[pixel]);
            if (curr_val == bits[pixel]) && (bits[pixel] == true) {
                collision_happened = true;
            }
        }
        return collision_happened;
    }
}
