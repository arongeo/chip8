#[path = "config.rs"]
mod config;

pub struct Stack {
    stack: [u16; config::CHIP8_STACK_SIZE],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; config::CHIP8_STACK_SIZE],
        }
    }
}
