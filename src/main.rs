// Copyright 2023 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// main.rs file
//

#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
mod chip8;
#[path = "cpu.rs"]
mod cpu;

extern crate minifb;
extern crate rodio;

use std::time::Duration;

use minifb::Window;
use minifb::WindowOptions;

fn main() {
    let filename = config::read_rom_filename();

    let mut window = match Window::new(format!("Chip-8 - {}", filename).as_str(), 64, 32, WindowOptions {
        scale: minifb::Scale::X8,
        borderless: false,
        ..WindowOptions::default()
    }) {
        Ok(win)     => win,
        Err(error)  => panic!("Couldn't create window instance!"),
    };

    window.limit_update_rate(Some(Duration::from_micros(2083)));

    let mut chip8cpu = cpu::Cpu::new(window, filename);
    chip8cpu.load_instructions();
    chip8cpu.start_execution();
}
