// Copyright 2022 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// chip8.rs file
//

#[path = "config.rs"]
pub mod config;
#[path = "memory.rs"]
pub mod memory;
#[path = "stack.rs"]
pub mod stack;
#[path = "registers.rs"]
pub mod registers;
#[path = "keyboard.rs"]
pub mod keyboard;
#[path = "display.rs"]
pub mod display;

pub struct Chip8 {
    pub memory: memory::Memory,
    pub stack: stack::Stack,
    pub registers: registers::Registers,
    pub keyboard: keyboard::Keyboard,
    pub display: display::Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            stack: stack::Stack::new(),
            registers: registers::Registers::new(),
            keyboard: keyboard::Keyboard::new(),
            display: display::Display::new(),
        }
    }

    pub fn stack_push(&mut self, value: u16) {
        self.registers.sp = self.registers.sp + 1;
        self.stack.stack[self.registers.sp] = value;
    }

    pub fn stack_pop(&mut self) -> u16 {
        let result = self.stack.stack[self.registers.sp];
        self.registers.sp = self.registers.sp - 1;
        result
    }
}
