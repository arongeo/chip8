#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
pub mod chip8;

pub struct Cpu {
    pub chip8: chip8::Chip8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            chip8: chip8::Chip8::new(),
        }
    }

    pub fn load(&mut self) {
        self.chip8.load_instructions();
        let instruction: u16 = (self.chip8.memory.ram[self.chip8.registers.pc as usize] as u16) << 8 | (self.chip8.memory.ram[(self.chip8.registers.pc+1) as usize]) as u16;
        let instruction_num: u16 = instruction >> 12;
        if instruction_num == 0x0 {
            self.chip8.display.clear()
        }
    }

}
