// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// stack.rs file
//

#[path = "config.rs"]
mod config;
#[path = "registers.rs"]
mod registers;

pub struct Stack {
    pub stack: [u16; config::CHIP8_STACK_SIZE],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; config::CHIP8_STACK_SIZE],
        }
    }
}
