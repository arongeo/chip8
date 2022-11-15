#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
pub mod chip8;

pub struct Instruction {
    pub nibbles:    [u8; 4],
    pub nnn:        u16,
    pub n:          u8,
    pub x:          u8,
    pub y:          u8,
    pub kk:         u8,
}

impl Instruction {
    pub fn new(instruction: u16) -> Self {
        let mut new_nibbles = [0u8; 4];
        new_nibbles[0]  =   (instruction >> 12) as u8;
        new_nibbles[1]  =   ((instruction & 0b0000111100000000) >> 8) as u8;
        new_nibbles[2]  =   ((instruction & 0b0000000011110000) >> 4) as u8;
        new_nibbles[3]  =   (instruction & 0b0000000000001111) as u8;
        let new_nnn     =   (instruction & 0b0000111111111111);         // Example: 1nnn
        let new_n       =   new_nibbles[3];                             // Example: Dxyn
        let new_x       =   new_nibbles[1];                             // Example: Fx07
        let new_y       =   new_nibbles[2];                             // Example: 5xy0
        let new_kk      =   ((new_nibbles[2] << 4) & new_nibbles[3]);   // Example: 7xkk
        Self {
            nibbles:    new_nibbles,
            nnn:        new_nnn,
            n:          new_n,
            x:          new_x,
            y:          new_y,
            kk:         new_kk,
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

    pub fn next_inst(&mut self) {
        self.chip8.registers.pc = self.chip8.registers.pc + 2;
    }

    pub fn skip_next_inst(&mut self) {
        self.chip8.registers.pc = self.chip8.registers.pc + 4;
    }

    pub fn load_instructions(&mut self) {
        self.chip8.memory.ram[0x200..0x1000].copy_from_slice(&self.chip8.romcartridge.rom);
    }

    pub fn get_instruction(&mut self) -> Instruction {
        let instruction: u16 = (self.chip8.memory.ram[self.chip8.registers.pc as usize] as u16) << 8 | (self.chip8.memory.ram[(self.chip8.registers.pc+1) as usize] as u16);
        Instruction::new(instruction)
    }

    pub fn execute_instruction(&mut self, instruction: &mut Instruction) {
        match instruction.nibbles {
            [0x0, 0x0, 0xE, 0x0]    => self.cls(),
            [0x0, 0x0, 0xE, 0xE]    => self.ret(),
            [0x1, _, _, _]          => self.jump(instruction.nnn),
            [0x2, _, _, _]          => self.call(instruction.nnn),
            [0x3, _, _, _]          => self.se_vx_byte(instruction.x, instruction.kk),
            [0x4, _, _, _]          => self.sne_vx_byte(instruction.x, instruction.kk),
            [0x5, _, _, 0x0]        => self.se_vx_vy(instruction.x, instruction.y),
            [0x6, _, _, _]          => self.ld_vx_byte(instruction.x, instruction.kk),
            _ => self.next_inst(),
        }
    }
    
    
    // Start of instruction set implementation

    fn cls(&mut self) {
        self.chip8.display.clear();
        self.next_inst();
    }

    fn ret(&mut self) {
        self.chip8.registers.pc = self.chip8.stack.stack[self.chip8.registers.sp];
        self.chip8.registers.sp = self.chip8.registers.sp - 1;
    }

    fn jump(&mut self, nnn: u16) {
        self.chip8.registers.pc = nnn;
        self.next_inst();
    }

    fn call(&mut self, nnn: u16) {
        self.chip8.registers.sp = self.chip8.registers.sp + 1;
        self.chip8.stack.stack[self.chip8.registers.sp] = self.chip8.registers.pc;
        self.chip8.registers.pc = nnn;
    } 

    fn se_vx_byte(&mut self, x: u8, kk: u8) {
        if self.chip8.registers.v[x as usize] == kk {
            self.skip_next_inst();
        } else {
            self.next_inst();
        }
    }

    fn sne_vx_byte(&mut self, x: u8, kk: u8) {
        if self.chip8.registers.v[x as usize] != kk {
            self.next_inst();
        } else {
            self.skip_next_inst();
        }
    }

    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.chip8.registers.v[x as usize] == self.chip8.registers.v[y as usize] {
            self.skip_next_inst();
        } else {
            self.next_inst();
        }
    }

    fn ld_vx_byte(&mut self, x: u8, kk: u8) {
        self.chip8.registers.v[x as usize] = kk;
        self.next_inst();
    }
}
