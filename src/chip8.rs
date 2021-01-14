use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Chip8 {
     vm: Chip8_VM,
}
// struct for the VM, contains relevent counters and memory
pub struct Chip8_VM {
     memory: [u8; 4096], // 4096 blocks of memory
     v: [u8; 16],        // 16 stored variables
     stack: [u8; 16],    // 16 nested levels for the stack
     sp: usize,          // stack pointer
     delay_timer: u8,    // count-down timer delayed actions
     sound_timer: u8,    // count-down timer for sounds
     opcode: u16,        // temporary storage for opcode that is currently being evaluated
     i: usize,           // 16 bit register for memory addresses
     pc: usize,          // program counter - where we are currently in memory
     gfx: [u8; 64 * 32], // storage of graphic state information
}

// generates the Chip8_VM
impl Chip8_VM {
     fn new() -> Chip8_VM {
          // construct initial state for the VM
          let mem: [u8; 4096] = [0; 4096];
          let reg: [u8; 16] = [0; 16];
          let st: [u8; 16] = [0; 16];
          let d_timer: u8 = 0;
          let s_timer: u8 = 0;
          let gx: [u8; 64 * 32] = [0; 64 * 32];
          Chip8_VM {
               memory: mem,
               v: reg,
               stack: st,
               sp: 0,
               delay_timer: d_timer,
               sound_timer: s_timer,
               opcode: 0,
               i: 0,
               pc: 512,
               gfx: gx,
          }
     }
}
// emulator
impl Chip8 {
     // create initial state for chip8
     pub fn initialize() -> Chip8 {
          Chip8 {
               vm: Chip8_VM::new(),
          }
     }
     pub fn load_game(&mut self, game: &str) {
          let mut file: String = String::from("src/games/");
          file.push_str(game);
          file.push_str(".rom");
          let f = File::open(file.to_string()).expect("unable to open or locate file"); // read file from games folder
          let reader = BufReader::new(f); // send file to buffer
          let mut pos = 0;
          for byte in reader.bytes() {
               // store game in memory
               self.vm.memory[pos + 512] = byte.expect("0");
               pos += 1;
          }
     }
     pub fn emulate_cycle(&mut self) -> i32 {
          let mut opcode: u16 = self.vm.memory[self.vm.pc].into();
          self.vm.opcode = opcode;
          opcode = (opcode << 8) ^ self.vm.memory[self.vm.pc + 1] as u16;
          self.vm.pc += 2;
          let mut c = 0;
          match opcode & 0xF000 {
               0x0000 => self.zero(), // 0NNN, calls function that handles opcodes that begin with 0
               0x1000 => self.vm.pc = (0x0FFF & opcode) as usize, // 0x1NNN, jump to NNN
               0x2000 => {
                    // 0x2NNN,call subroutine at NNN
                    self.vm.sp += 1;
                    self.vm.stack[self.vm.sp] = self.vm.pc as u8;
               }
               0x3000 => {
                    // 0x3XNN, check if VX == NN. skip next instruction if true.
                    if self.vm.v[(0x0F00 & opcode) as usize] == (0x00FF & opcode) as u8 {
                         self.vm.pc += 2;
                    }
               }
               0x4000 => {
                    // 0x4XNN, check if VX != NN. skip next instruction if true.
                    if self.vm.v[(0x0F00 & opcode) as usize] != (0x00FF & opcode) as u8 {
                         self.vm.pc += 2;
                    }
               }
               0x5000 => {
                    // 0x5XY0, skip next instruction if VX == VY
                    if self.vm.v[(0x0F00 & opcode) as usize]
                         == self.vm.v[(0x0F00 & opcode) as usize]
                    {
                         self.vm.pc += 2;
                    }
               }
               0x6000 => {
                    // 0x6XNN, set VX = NN
                    let x = (opcode & 0x0F00) as usize >> 8;
                    let n: u8 = (opcode & 0x00FF) as u8;
                    self.vm.v[x] = n;
                    // println!("x: {}, n: {}", x, n);
               }
               0x7000 => {
                    // 0x7XNN, add NN to VX
                    let n = 0x00FF;
                    self.vm.v[(0x0F00 >> 16) as usize] += n as u8;
               }
               0x8000 => {
                    // calls function eight() that handles 0x8XXX opcodes
                    self.eight();
               }
               _ => {
                    c += 1;
               }
          }
          c
     }
     fn zero(&mut self) {
          // opcodes that begin with 0
          let code = 0x0FFF & self.vm.opcode;
          match code {
               0x00E0 => self.clear_screen(), // clear screen
               0x00EE => {
                    // return from subroutine
                    self.vm.stack[self.vm.sp] = 0; // clear stack
                    self.vm.sp -= 1; // go back one level in the stack
                    self.vm.pc = (self.vm.stack[self.vm.sp] + 2) as usize; // set next opcode to be the next opcode in memory after the prior call in the stack
               }
               _ => self.vm.pc = code as usize,
          }
     }
     fn eight(&mut self) {
          // opcodes that begin with 8
     }
     fn clear_screen(&mut self) {
          // clear graphics screen
     }
}
