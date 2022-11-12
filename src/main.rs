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
use std::time::Duration;

use cpu::chip8::keyboard::VKeys;

fn main() {
    let mut chip8cpu = cpu::Cpu::new();

    let sdl_context = sdl2::init().unwrap();
    let v_subsys = sdl_context.video().unwrap();

    let mut window = v_subsys.window("Chip-8", (config::CHIP8_DISPLAY_WIDTH * 10) as u32, (config::CHIP8_DISPLAY_HEIGHT * 10) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    chip8cpu.chip8.display.set_pixel(0, 0, true);
    chip8cpu.load();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        chip8cpu.chip8.display.render(&mut canvas);
        chip8cpu.chip8.keyboard.check_keys(&mut event_pump);
        if chip8cpu.chip8.keyboard.get_key_status(VKeys::Key1) == true {
            chip8cpu.chip8.display.set_pixel(0, 2, true);
        }
        if chip8cpu.chip8.keyboard.get_key_status(VKeys::Key2) == true {
            chip8cpu.chip8.display.set_pixel(0, 2, false);
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
