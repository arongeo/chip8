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
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(event_pump_in: EventPump) -> Self {
        Self {
            keys: [false; config::CHIP8_KEYBOARD_SIZE],
            event_pump: event_pump_in,
        }
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
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    return false;
                },
                _ => {
                    return true;
                }
            }
        }
        return true;
    }

    pub fn check_keys(&mut self) {
        let pressed_keys: Vec<Keycode> = self.event_pump
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
