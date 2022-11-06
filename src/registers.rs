// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// registers.rs file
//

#[path = "config.rs"]
mod config;

pub struct Registers {
    pub v: [u8; config::CHIP8_REGISTERS_NUM],
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: usize,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0; config::CHIP8_REGISTERS_NUM],
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        }
    }
}
