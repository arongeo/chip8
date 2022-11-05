#[path = "config.rs"]
mod config;

pub struct Registers {
    v: [u8; config::CHIP8_REGISTERS_NUM],
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0; config::CHIP8_REGISTERS_NUM],
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        }
    }
}
