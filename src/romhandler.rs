// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// romhandler.rs file
//

#[path = "config.rs"]
mod config;

use std::fs;
use std::path::Path;

pub struct RomCartridge {
    pub rom: [u8; config::CHIP8_INSTRUCTIONS_SIZE],
}

impl RomCartridge {
    pub fn new(romfilename: String) -> Self {
        let (bytes, bytes_size) = Self::load_bytes_from_rom(romfilename);
        Self {
            rom: bytes,
        }
    }


    fn load_bytes_from_rom(romfilename: String) -> ([u8; config::CHIP8_INSTRUCTIONS_SIZE], usize) {
        let bytes_buf: Vec<u8> = fs::read(Path::new(romfilename.as_str())).unwrap(); 
        let mut bytes = [0u8; config::CHIP8_INSTRUCTIONS_SIZE];
        if bytes_buf.len() > config::CHIP8_INSTRUCTIONS_SIZE {
            panic!("Error: ROM bytes exceed memory size");
        }
        for i in 0..bytes_buf.len() {
            bytes[i] = bytes_buf[i];
        }
        (bytes, bytes.len())
    }

}
