#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
pub mod chip8;

struct Instruction {
    id:     u8,
    nnn:    u16,
    n:      u8,
    x:      u8,
    y:      u8,
    kk:     u8
}

impl Instruction {
    pub fn new(instruction: u16) -> Self {
        let new_id  =   (instruction >> 12) as u8;
        let new_nnn =   instruction & 0b0000111111111111;           // Example: 1nnn
        let new_n   =   (instruction & 0b0000000000001111) as u8;   // Example: Dxyn
        let new_x   =   (instruction & 0b0000111100000000) as u8;   // Example: Fx07
        let new_y   =   (instruction & 0b0000000011110000) as u8;   // Example: 5xy0
        let new_kk  =   (instruction & 0b0000000011111111) as u8;   // Example: 7xkk
        Self {
            id:     new_id,
            nnn:    new_nnn,
            n:      new_n,
            x:      new_x,
            y:      new_y,
            kk:     new_kk,
        }
    }
}

pub struct Cpu {
    pub chip8: chip8::Chip8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            chip8: chip8::Chip8::new(),
        }
    }

    pub fn load_instructions(&mut self) {
        self.chip8.memory.ram[0x200..0x1000].copy_from_slice(&self.chip8.romcartridge.rom);
    }

    pub fn get_instruction(&mut self) -> Instruction {
        let instruction: u16 = (self.chip8.memory.ram[self.chip8.registers.pc as usize] as u16) << 8 | (self.chip8.memory.ram[(self.chip8.registers.pc+1) as usize] as u16);
        self.chip8.registers.sp = self.chip8.registers.sp + 2;
        Instruction::new(instruction)
    }

    pub fn execute_instruction(&mut self, instruction: &mut Instruction) {
        
    }
}
