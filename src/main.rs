#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
mod chip8;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let mut chip_8: chip8::Chip8 = chip8::Chip8::new();

    let sdl_context = sdl2::init().unwrap();
    let v_subsys = sdl_context.video().unwrap();

    let mut window = v_subsys.window("Chip-8", (config::CHIP8_DISPLAY_WIDTH * 10).try_into().unwrap(), (config::CHIP8_DISPLAY_HEIGHT * 10).try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    chip_8.display.set_pixel(0, 0, true);
    chip_8.display.set_pixel(0, 1, true);
    'running: loop {
        chip_8.display.render(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    chip_8.display.set_pixel(0, 2, true);
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    chip_8.display.set_pixel(0, 2, false);
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
