#[path = "chip8.rs"]
mod chip8;

fn main() {
    let mut chip_8: chip8::Chip8 = chip8::Chip8::new();
    println!("{}", chip_8.memory.ram[0]);
}
