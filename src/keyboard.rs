// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// keyboard.rs file
//

#[path = "config.rs"]
mod config;

pub struct Keyboard {
    keys: [bool; config::CHIP8_KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; config::CHIP8_KEYBOARD_SIZE],
        }
    }

    pub fn key_down(&mut self, i: usize) {
        self.keys[i] = true;
    }

    pub fn key_up(&mut self, i: usize) {
        self.keys[i] = false;
    }
}