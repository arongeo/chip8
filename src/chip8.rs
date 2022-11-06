#[path = "config.rs"]
mod config;
#[path = "memory.rs"]
mod memory;
#[path = "stack.rs"]
mod stack;
#[path = "registers.rs"]
mod registers;
#[path = "keyboard.rs"]
mod keyboard;

pub struct Chip8 {
    pub memory: memory::Memory,
    pub stack: stack::Stack,
    pub registers: registers::Registers,
    pub keyboard: keyboard::Keyboard,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            stack: stack::Stack::new(),
            registers: registers::Registers::new(),
            keyboard: keyboard::Keyboard::new(),
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
