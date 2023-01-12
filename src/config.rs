// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// config.rs file
//

pub const CHIP8_MEMORY_SIZE: usize = 4096;
pub const CHIP8_DISPLAY_WIDTH: usize = 64;
pub const CHIP8_DISPLAY_HEIGHT: usize = 32;
pub const CHIP8_REGISTERS_NUM: usize = 16;
pub const CHIP8_STACK_SIZE: usize = 16;
pub const CHIP8_KEYBOARD_SIZE: usize = 16;
pub const CHIP8_CHARSET_LEN: usize = 80;
pub const CHIP8_INSTRUCTIONS_SIZE: usize = 0xE00;

pub fn get_bit_values(n: u8) -> [bool; 8] {
    let mut and_op: u8 = 128;
    let mut bit_stats: [bool; 8] = [false; 8];
    for bit_index in (0..8).rev() {
        if ((n & and_op) >> bit_index) == 1 {
            bit_stats[7 - bit_index] = true; 
        } else {
            bit_stats[7 - bit_index] = false;
        }
        and_op = and_op / 2;
    }
    return bit_stats;
}
