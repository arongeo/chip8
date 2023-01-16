// Copyright 2023 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// io.rs file
//

#[path = "config.rs"]
mod config;

use minifb::{Key, KeyRepeat, WindowOptions};

pub enum VKeys {
    Key1,
    Key2,
    Key3,
    KeyC,
    Key4,
    Key5,
    Key6,
    KeyD,
    Key7,
    Key8,
    Key9,
    KeyE,
    KeyA,
    Key0,
    KeyB,
    KeyF,
}

const PIXEL_ON:  u32    = 0b00000000111111111111111111111111;
const PIXEL_OFF: u32    = 0;

pub struct IO {
    window: crate::minifb::Window,
    keys: [bool; config::CHIP8_KEYBOARD_SIZE],
    pub pixels: [[bool; config::CHIP8_DISPLAY_HEIGHT]; config::CHIP8_DISPLAY_WIDTH],
}

impl IO {
    pub fn new(win: crate::minifb::Window) -> Self {
        Self {
            keys: [false; config::CHIP8_KEYBOARD_SIZE],
            pixels: [[false; config::CHIP8_DISPLAY_HEIGHT]; config::CHIP8_DISPLAY_WIDTH],
            window: win,
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

    pub fn render(&mut self) {
        let mut buffer: [u32; config::CHIP8_DISPLAY_WIDTH * config::CHIP8_DISPLAY_HEIGHT] = [PIXEL_OFF; config::CHIP8_DISPLAY_WIDTH * config::CHIP8_DISPLAY_HEIGHT];
        let mut pixel_coordinate: usize = 0;
        for x in 0..config::CHIP8_DISPLAY_WIDTH {
            for y in 0..config::CHIP8_DISPLAY_HEIGHT {
                pixel_coordinate = (y * config::CHIP8_DISPLAY_WIDTH) + x;
                if self.get_pixel(x, y) == true {
                    buffer[pixel_coordinate] = PIXEL_ON;
                }
            }
        }
        self.window.update_with_buffer(&buffer, 64, 32);
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

    pub fn get_key_status_from_vkey(&mut self, key: VKeys) -> bool {
        self.keys[key as usize]
    }

    pub fn get_key_status_from_num(&mut self, n: u8) -> bool {
        match n {
            0x1 => return self.get_key_status_from_vkey(VKeys::Key1),
            0x2 => return self.get_key_status_from_vkey(VKeys::Key2),
            0x3 => return self.get_key_status_from_vkey(VKeys::Key3),
            0xC => return self.get_key_status_from_vkey(VKeys::KeyC),
            0x4 => return self.get_key_status_from_vkey(VKeys::Key4),
            0x5 => return self.get_key_status_from_vkey(VKeys::Key5),
            0x6 => return self.get_key_status_from_vkey(VKeys::Key6),
            0xD => return self.get_key_status_from_vkey(VKeys::KeyD),
            0x7 => return self.get_key_status_from_vkey(VKeys::Key7),
            0x8 => return self.get_key_status_from_vkey(VKeys::Key8),
            0x9 => return self.get_key_status_from_vkey(VKeys::Key9),
            0xE => return self.get_key_status_from_vkey(VKeys::KeyE),
            0xA => return self.get_key_status_from_vkey(VKeys::KeyA),
            0x0 => return self.get_key_status_from_vkey(VKeys::Key0),
            0xB => return self.get_key_status_from_vkey(VKeys::KeyB),
            0xF => return self.get_key_status_from_vkey(VKeys::KeyF),
            _   => panic!("ERROR: Couldn't parse KEYID"),
        };
    }

    pub fn poll_quit(&mut self) -> bool {
        self.render();
        if (!self.window.is_open()) | (!self.window.is_key_down(Key::Escape)) {
            return true;
        } else {
            return false;
        }
    }

    pub fn check_keys(&mut self) {
        self.render();

        self.keys = [false; config::CHIP8_KEYBOARD_SIZE];

        self.window.get_keys().iter().for_each(|key| 
            match key {
                Key::Key1 => self.keys[VKeys::Key1 as usize] = true,
                Key::Key2 => self.keys[VKeys::Key2 as usize] = true,
                Key::Key3 => self.keys[VKeys::Key3 as usize] = true,
                Key::Key4 => self.keys[VKeys::KeyC as usize] = true,
                Key::Q    => self.keys[VKeys::Key4 as usize] = true,
                Key::W    => self.keys[VKeys::Key5 as usize] = true,
                Key::E    => self.keys[VKeys::Key6 as usize] = true,
                Key::R    => self.keys[VKeys::KeyD as usize] = true,
                Key::A    => self.keys[VKeys::Key7 as usize] = true,
                Key::S    => self.keys[VKeys::Key8 as usize] = true,
                Key::D    => self.keys[VKeys::Key9 as usize] = true,
                Key::F    => self.keys[VKeys::KeyE as usize] = true,
                Key::Z    => self.keys[VKeys::KeyA as usize] = true,
                Key::X    => self.keys[VKeys::Key0 as usize] = true,
                Key::C    => self.keys[VKeys::KeyB as usize] = true,
                Key::V    => self.keys[VKeys::KeyF as usize] = true,
                _ => (),
        });
    }

    pub fn wait_for_key(&mut self) -> u8 {
        self.render();
        let mut key: Result<u8, ()> = Err(());
        let mut keys = self.window.get_keys();
        loop {
            while (keys.len() == 0) {
                self.render();
                keys = self.window.get_keys();
            }
            key = match keys[0] {
                Key::Key1   => Ok(0x1),
                Key::Key2   => Ok(0x2),
                Key::Key3   => Ok(0x3),
                Key::Key4   => Ok(0xC),
                Key::Q      => Ok(0x4),
                Key::W      => Ok(0x5),
                Key::E      => Ok(0x6),
                Key::R      => Ok(0xD),
                Key::A      => Ok(0x7),
                Key::S      => Ok(0x8),
                Key::D      => Ok(0x9),
                Key::F      => Ok(0xE),
                Key::Z      => Ok(0xA),
                Key::X      => Ok(0x0),
                Key::C      => Ok(0xB),
                Key::V      => Ok(0xF),
                _           => Err(()),
            };
            if key != Err(()) {
                return key.unwrap();
            }
        }
    } 
}
