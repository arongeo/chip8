#[path = "config.rs"]
mod config;
#[path = "chip8.rs"]
pub mod chip8;

use rand::Rng;
use crate::chip8::io::VKeys;
use std::time::Duration;

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
        let new_kk      =   (new_nnn & 0b0000000011111111) as u8;      // Example: 7xkk
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
    pub fn new(window: crate::minifb::Window, romfile: String) -> Self {
        Self {
            chip8: chip8::Chip8::new(window, romfile),
        }
    }

    pub fn start_execution(&mut self) {
        let mut next_instruction = self.get_instruction();
        self.chip8.io.render();
        std::thread::sleep(Duration::from_millis(20));
        while (self.chip8.registers.pc >= 0x200) && (self.chip8.io.poll_quit() != false) {
            if self.chip8.registers.pc > 0xFFD {
                println!("ERROR: The code you're running tried to write out of memory bounds!");
                while (self.chip8.io.poll_quit() != false) {}
                break;
            }
            self.execute_instruction(next_instruction);
            next_instruction = self.get_instruction();
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

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.nibbles {
            [0x0, 0x0, 0xE, 0x0]    => self.cls(),
            [0x0, 0x0, 0xE, 0xE]    => self.ret(),
            [0x1, _, _, _]          => self.jump(instruction.nnn),
            [0x2, _, _, _]          => self.call(instruction.nnn),
            [0x3, _, _, _]          => self.se_vx_byte(instruction.x, instruction.kk),
            [0x4, _, _, _]          => self.sne_vx_byte(instruction.x, instruction.kk),
            [0x5, _, _, 0x0]        => self.se_vx_vy(instruction.x, instruction.y),
            [0x6, _, _, _]          => self.ld_vx_byte(instruction.x, instruction.kk),
            [0x7, _, _, _]          => self.add_vx_byte(instruction.x, instruction.kk),
            [0x8, _, _, 0x0]        => self.ld_vx_byte(instruction.x, instruction.y),
            [0x8, _, _, 0x1]        => self.or_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0x2]        => self.and_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0x3]        => self.xor_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0x4]        => self.add_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0x5]        => self.sub_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0x6]        => self.shr_vx(instruction.x),
            [0x8, _, _, 0x7]        => self.subn_vx_vy(instruction.x, instruction.y),
            [0x8, _, _, 0xE]        => self.shl_vx(instruction.x),
            [0x9, _, _, 0x0]        => self.sne_vx_vy(instruction.x, instruction.y),
            [0xA, _, _, _]          => self.ld_i_addr(instruction.nnn),
            [0xB, _, _, _]          => self.jp_v0_nnn(instruction.nnn),
            [0xC, _, _, _]          => self.rnd_vx_byte(instruction.x, instruction.kk),
            [0xD, _, _, _]          => self.drw_vx_vy_n(instruction.x, instruction.y, instruction.n),
            [0xE, _, 0x9, 0xE]      => self.skp_vx(instruction.x),
            [0xE, _, 0xA, 0x1]      => self.sknp_vx(instruction.x),
            _ => self.next_inst(),
        }
    }
    
    
    // Start of instruction set implementation

    fn cls(&mut self) {
        self.chip8.io.clear();
        self.next_inst();
    }

    fn ret(&mut self) {
        self.chip8.registers.pc = self.chip8.stack.stack[self.chip8.registers.sp];
        self.chip8.registers.sp = self.chip8.registers.sp - 1;
    }

    fn jump(&mut self, nnn: u16) {
        self.chip8.registers.pc = nnn;
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

    fn add_vx_byte(&mut self, x: u8, kk: u8) {
        self.chip8.registers.v[x as usize] = self.chip8.registers.v[x as usize] + kk;
        self.next_inst();
    }

    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.chip8.registers.v[x as usize] = self.chip8.registers.v[y as usize];
        self.next_inst();
    }

    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.chip8.registers.v[x as usize] |= self.chip8.registers.v[y as usize];
        self.next_inst();
    }

    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.chip8.registers.v[x as usize] &= self.chip8.registers.v[y as usize];
        self.next_inst();
    }

    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.chip8.registers.v[x as usize] ^= self.chip8.registers.v[y as usize];
        self.next_inst();
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let mut sum_wo_filter = (self.chip8.registers.v[x as usize] + self.chip8.registers.v[y as usize]) as u16;
        if (sum_wo_filter > 255) {
            self.chip8.registers.v[x as usize] = (0b0000000011111111 & sum_wo_filter) as u8;
            self.chip8.registers.v[0xF] = 1;
        }
        else {
            self.chip8.registers.v[x as usize] = sum_wo_filter as u8;
        }
        self.next_inst();
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        if self.chip8.registers.v[x as usize] > self.chip8.registers.v[y as usize] {
            self.chip8.registers.v[0xF] = 1;
            self.chip8.registers.v[x as usize] = self.chip8.registers.v[x as usize] - self.chip8.registers.v[y as usize];
        } else {
            self.chip8.registers.v[0xF] = 0;
            self.chip8.registers.v[x as usize] = self.chip8.registers.v[y as usize] - self.chip8.registers.v[x as usize]; 
        }
        self.next_inst();
    }

    fn shr_vx(&mut self, x: u8) {
        if (self.chip8.registers.v[x as usize] & 0b00000001) == 1 {
            self.chip8.registers.v[0xF] = 1;
        } else {
            self.chip8.registers.v[0xF] = 0;
        }
        self.chip8.registers.v[x as usize] = self.chip8.registers.v[x as usize] / 2;
        self.next_inst();
    }

    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        if (self.chip8.registers.v[y as usize] > self.chip8.registers.v[x as usize]) {
            self.chip8.registers.v[0xF] = 1;
        } else {
            self.chip8.registers.v[0xF] = 0;
        }
        self.chip8.registers.v[x as usize] = self.chip8.registers.v[y as usize] - self.chip8.registers.v[x as usize];
        self.next_inst();
    }

    fn shl_vx(&mut self, x: u8) {
        if (self.chip8.registers.v[x as usize] & 0b10000000) > 0 {
            self.chip8.registers.v[0xF] = 1; 
        } else {
            self.chip8.registers.v[0xF] = 0;
        }
        self.chip8.registers.v[x as usize] = self.chip8.registers.v[x as usize] * 2;
        self.next_inst();
    }
    
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.chip8.registers.v[x as usize] != self.chip8.registers.v[y as usize] {
           self.skip_next_inst();
        } else {
            self.next_inst();
        }
    }

    fn ld_i_addr(&mut self, nnn: u16) {
        self.chip8.registers.i = nnn;
        self.next_inst();
    }

    fn jp_v0_nnn(&mut self, nnn: u16) {
        self.chip8.registers.pc = nnn + self.chip8.registers.v[0x0] as u16;
    }

    fn rnd_vx_byte(&mut self, x: u8, byte: u8) {
        let mut rng = rand::thread_rng();
        let rand_num: u8 = rng.gen_range(0..255);
        self.chip8.registers.v[x as usize] = byte & rand_num;
        self.next_inst();
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn drw_vx_vy_n(&mut self, x: u8, y: u8, n: u8) {
        self.chip8.registers.v[0xF] = 0;
        for mem_addr_count in 0..n {
            if self.chip8.io.draw_byte((self.chip8.registers.v[x as usize]) as usize, (self.chip8.registers.v[y as usize] + mem_addr_count) as usize, self.chip8.memory.ram[(self.chip8.registers.i + mem_addr_count as u16) as usize]) == true {
                self.chip8.registers.v[0xF] = 1;
            }
        }
        self.next_inst();
    }

    fn skp_vx(&mut self, x: u8) {
        self.chip8.io.check_keys();
        let key_status = self.chip8.io.get_key_status_from_num(x);
        if key_status == true {
            self.skip_next_inst();
        } else {
            self.next_inst();
        }
    }

    fn sknp_vx(&mut self, x: u8) {
        self.chip8.io.check_keys();
        let key_status = self.chip8.io.get_key_status_from_num(x);
        if key_status == false {
            self.skip_next_inst();
        } else {
            self.next_inst();
        }
    }
}
