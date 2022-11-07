// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// keyboard.rs file
//

#[path = "config.rs"]
mod config;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;

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

pub struct Keyboard {
    keys: [bool; config::CHIP8_KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; config::CHIP8_KEYBOARD_SIZE],
        }
    }

    pub fn get_key_status(&mut self, key: VKeys) -> bool {
        self.keys[key as usize]
    }

    pub fn check_keys(&mut self, event_pump: &mut EventPump) {
        let pressed_keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        self.keys = [false; config::CHIP8_KEYBOARD_SIZE];

        for key in pressed_keys {
            match key {
                Keycode::Num1 => self.keys[VKeys::Key1 as usize] = true,
                Keycode::Num2 => self.keys[VKeys::Key2 as usize] = true,
                Keycode::Num3 => self.keys[VKeys::Key3 as usize] = true,
                Keycode::Num4 => self.keys[VKeys::KeyC as usize] = true,
                Keycode::Q    => self.keys[VKeys::Key4 as usize] = true,
                Keycode::W    => self.keys[VKeys::Key5 as usize] = true,
                Keycode::E    => self.keys[VKeys::Key6 as usize] = true,
                Keycode::R    => self.keys[VKeys::KeyD as usize] = true,
                Keycode::A    => self.keys[VKeys::Key7 as usize] = true,
                Keycode::S    => self.keys[VKeys::Key8 as usize] = true,
                Keycode::D    => self.keys[VKeys::Key9 as usize] = true,
                Keycode::F    => self.keys[VKeys::KeyE as usize] = true,
                Keycode::Z    => self.keys[VKeys::KeyA as usize] = true,
                Keycode::X    => self.keys[VKeys::Key0 as usize] = true,
                Keycode::C    => self.keys[VKeys::KeyB as usize] = true,
                Keycode::V    => self.keys[VKeys::KeyF as usize] = true,
                _ => (),
            }
        }
    }
}
