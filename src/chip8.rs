use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Chip8 {
     vm: Chip8_VM,
}

pub struct Chip8_VM {
     memory: [u8; 4096],
     v: [u8; 16],
     stack: [u8; 16],
     sp: u8,
     delay_timer: u8,
     sound_timer: u8,
     opcode: u16,
     i: u16,
     pc: u16,
     gfx: [u8; 64 * 32],
}
impl Chip8_VM {
     fn new() -> Chip8_VM {
          // construct initial state for the VM
          let mut mem: [u8; 4096] = [0; 4096];
          let mut reg: [u8; 16] = [0; 16];
          let mut st: [u8; 16] = [0; 16];
          let mut d_timer: u8 = 0;
          let mut s_timer: u8 = 0;
          let mut gx: [u8; 64 * 32] = [0; 64 * 32];
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

impl Chip8 {
     pub fn initialize() -> Chip8 {
          Chip8 {
               vm: Chip8_VM::new(),
          }
     }
     pub fn load_game(&mut self, game: &str) {
          let mut file: String = String::from("src/games/");
          file.push_str(game);
          file.push_str(".rom");
          let f = File::open(file.to_string()).expect("unable to open or locate file");
          let mut reader = BufReader::new(f);
          let mut pos = 0;
          for byte in reader.bytes() {
               self.vm.memory[pos + 512] = byte.expect("404");
               pos += 1;
          }
          println!("{:?}", self.vm.memory);
     }
     fn emulate_cycle(&mut self) {}
}
