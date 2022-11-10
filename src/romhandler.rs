#[path = "config.rs"]
mod config;

use std::fs;
use std::path::Path;

fn printhelp() {
    println!("Chip-8 Interpreter written in Rust");
    println!("\nMade by arongeo");
    println!("https://arongeo.com");
    println!("Usage: ");
    println!("chip8 <path/to/rom>");

}

pub struct RomCartridge {
    pub rom: [u8; config::CHIP8_INSTRUCTIONS_SIZE],
}

impl RomCartridge {
    pub fn new() -> Self {
        let (bytes, bytes_size) = Self::load_bytes_from_rom();
        Self {
            rom: bytes,
        }
    }

    fn read_rom_filename() -> String {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 2 {
            args[1].clone()
        } else {
            printhelp();
            std::process::exit(0);
        }
    }

    fn load_bytes_from_rom() -> ([u8; config::CHIP8_INSTRUCTIONS_SIZE], usize) {
        let bytes_buf: Vec<u8> = fs::read(Path::new(Self::read_rom_filename().as_str())).unwrap();
        let mut bytes = [0u8; config::CHIP8_INSTRUCTIONS_SIZE];
        if bytes_buf.len() > config::CHIP8_INSTRUCTIONS_SIZE {
            panic!("Error: ROM bytes exceed memory size");
        }
        for i in 0..bytes_buf.len() {
            bytes[i] = bytes_buf[i];
        }
        (bytes, bytes.len())
    }

}
