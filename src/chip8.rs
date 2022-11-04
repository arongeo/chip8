mod config;

pub struct Chip8 {
    memory: [u8; config::CHIP8_MEMORY_SIZE],
    registers: [u8; config::CHIP8_REGISTERS_NUM],
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; config::CHIP8_STACK_SIZE],
    keyboard: [bool; config::CHIP8_KEYBOARD_SIZE],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            
        }
    }
}
