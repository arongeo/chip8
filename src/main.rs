// Copyright 2022 - https://github.com/arongeo
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

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use cpu::chip8::keyboard::VKeys;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let v_subsys = sdl_context.video().unwrap();

    let mut window = v_subsys.window("Chip-8", (config::CHIP8_DISPLAY_WIDTH * 10) as u32, (config::CHIP8_DISPLAY_HEIGHT * 10) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut chip8cpu = cpu::Cpu::new(event_pump, canvas);
    chip8cpu.load_instructions();
    chip8cpu.start_execution();
}
