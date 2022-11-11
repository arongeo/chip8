#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
mod chip8;

pub struct Processor {
    chip8: chip8::Chip8,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            chip8: chip8::Chip8::new(),
        }
    }

    pub fn load(&mut self, )

}