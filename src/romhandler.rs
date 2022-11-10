#[path = "config.rs"]
mod config;

use std::fs;
use std::path::Path;

pub fn load_bytes_from_rom(filename: &str) -> [u8; 0xE00] {
    let mut bytes: Vec<u8> = fs::read(Path::new(filename)).unwrap();
    let mut array_bytes = [0u8; 0xE00];
    for i in 0..bytes.len() {
        
    }
}