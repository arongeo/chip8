// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// memory.rs file
//

#[path = "config.rs"]
mod config;
#[path = "charset.rs"]
mod charset;
#[path = "romhandler.rs"]
mod romhandler;

pub struct Memory {
    pub ram: [u8; config::CHIP8_MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut ram: [u8; config::CHIP8_MEMORY_SIZE] = [0; config::CHIP8_MEMORY_SIZE];
        // loading the character set into the first 512 bytes
        ram[..config::CHIP8_CHARSET_LEN].clone_from_slice(&charset::CHIP8_CHARSET);
        Self {
            ram: ram,
        }
    }

    pub fn load_instructions(&mut self, romcart: romhandler::RomCartridge) {
        self.ram[0x200..0xFFF].clone_from_slice(&romcart.rom);
    }
}
