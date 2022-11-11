#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
mod chip8;

pub struct Cpu {
    chip8: chip8::Chip8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            chip8: chip8::Chip8::new(),
        }
    }

    pub fn load(&mut self) {
        self.chip8.memory.load_instructions(self.chip8.romcartridge);
        let instruction: u16 = (self.chip8.memory.ram[self.chip8.registers.pc as usize] as u16) << 8 | (self.chip8.memory.ram[(self.chip8.registers.pc+1) as usize]);
        
    }

}