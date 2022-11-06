#[path = "chip8.rs"]
mod chip8;

fn main() {
    let mut chip_8: chip8::Chip8 = chip8::Chip8::new();
    chip_8.stack_push(1);
    assert_eq!(1, chip_8.stack_pop());
    assert_eq!(0, chip_8.memory.ram[251]);
}
