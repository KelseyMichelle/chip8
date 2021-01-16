#![feature(array_map)]
extern crate sdl2;

mod key_check;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

const HEIGHT: u32 = 32;
const WIDTH: u32 = 64;
const PIXEL_SIZE: u32 = 16;

pub struct Chip8 {
     vm: Chip8_VM,
}
// struct for the VM, contains relevent counters and memory
pub struct Chip8_VM {
     sdl_context: sdl2::Sdl,
     keypress: u8,
     canvas: Canvas<Window>,
     name: String,
     memory: [u8; 4096],                   // 4096 blocks of memory
     v: [u8; 16],                          // 16 stored variables
     stack: [u16; 16],                     // 16 nested levels for the stack
     sp: usize,                            // stack pointer
     delay_timer: u8,                      // count-down timer delayed actions
     sound_timer: u8,                      // count-down timer for sounds
     opcode: u16, // temporary storage for opcode that is currently being evaluated
     i: usize,    // 16 bit register for memory addresses
     pc: usize,   // program counter - where we are currently in memory
     gfx: [u8; (WIDTH * HEIGHT) as usize], // storage of graphic state information
}

const FONTSET: [u8; 80] = [
     0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
     0x20, 0x60, 0x20, 0x20, 0x70, // 1
     0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
     0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
     0x90, 0x90, 0xF0, 0x10, 0x10, // 4
     0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
     0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
     0xF0, 0x10, 0x20, 0x40, 0x40, // 7
     0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
     0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
     0xF0, 0x90, 0xF0, 0x90, 0x90, // A
     0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
     0xF0, 0x80, 0x80, 0x80, 0xF0, // C
     0xE0, 0x90, 0x90, 0x90, 0xE0, // D
     0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
     0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// generates the Chip8 VM
impl Chip8_VM {
     fn new() -> Chip8_VM {
          // construct initial state for the VM
          let mem: [u8; 4096] = [0; 4096];
          let reg: [u8; 16] = [0; 16];
          let st: [u16; 16] = [0; 16];
          let d_timer: u8 = 0;
          let s_timer: u8 = 0;
          let gx: [u8; (WIDTH * HEIGHT) as usize] = [0; (WIDTH * HEIGHT) as usize];
          let sdl_con = sdl2::init().unwrap();
          let can: Canvas<Window> = Chip8::make_canvas(&sdl_con, "Chip-8 Emulator");
          Chip8_VM {
               sdl_context: sdl_con,
               keypress: 17,
               name: String::from(""),
               canvas: can,
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
          self.vm.name = String::from(game);
          let mut file: String = String::from("src/roms/");
          file.push_str(game);
          // println!("{}", file);
          let f = File::open(file.to_string()).expect("unable to open or locate file"); // read file from games folder
          let reader = BufReader::new(f); // send file to buffers
          let mut pos = 0;
          for byte in reader.bytes() {
               // store game in memory
               self.vm.memory[pos + 0x200] = byte.expect("0");
               pos += 1;
          }
          pos = 0;
          for byte in FONTSET.iter() {
               self.vm.memory[pos + 0x50] = *byte;
               pos += 1;
          }
     }
     // processes current opcode that is stored at the memory address that pc is pointing to
     pub fn emulate_cycle(&mut self) {
          if self.vm.delay_timer >= 1 {
               self.vm.delay_timer -= 1;
          }
          if self.vm.sound_timer > 1 {
               self.vm.sound_timer -= 1;
          } else if self.vm.sound_timer == 1 {
               self.vm.sound_timer -= 1;
               // println!("beep!!!!!!!!!!!!!!");
          }
          let mut rng = rand::thread_rng();
          let mut opcode: u16 = self.vm.memory[self.vm.pc].into();
          opcode = (opcode << 8) ^ self.vm.memory[self.vm.pc + 1] as u16;
          self.vm.opcode = opcode;
          // println!("0x{:x}", opcode);
          self.vm.pc += 2;
          let mut c = 0;
          let x: usize = ((0x0F00 & self.vm.opcode) >> 8) as usize;
          if x == 14 {
               // println!("E REFERENCING ADDRESS {:x}", opcode);
          }
          match opcode & 0xF000 {
               0x0000 => self.zero(), // 0NNN, calls function that handles opcodes that begin with 0
               0x1000 => self.vm.pc = (0x0FFF & opcode) as usize, // 0x1NNN, jump to NNN
               0x2000 => {
                    // 0x2NNN,call subroutine at NNN
                    self.vm.stack[self.vm.sp] = self.vm.pc as u16;
                    self.vm.sp += 1;
                    self.vm.stack[self.vm.sp] = (opcode & 0x0FFF) as u16;
                    self.vm.pc = 0x0FFF & opcode as usize;
               }
               0x3000 => {
                    // 0x3XNN, check if VX == NN. skip next instruction if true.
                    if self.vm.v[x] == (0x00FF & opcode) as u8 {
                         self.vm.pc += 2;
                    }
               }
               0x4000 => {
                    // 0x4XNN, check if VX != NN. skip next instruction if true.
                    if self.vm.v[x] != (0x00FF & opcode) as u8 {
                         self.vm.pc += 2;
                    }
               }
               0x5000 => {
                    // 0x5XY0, skip next instruction if VX == VY
                    let y: usize = ((0x00F0 & self.vm.opcode) >> 4) as usize;
                    if self.vm.v[x] == self.vm.v[y] {
                         self.vm.pc += 2;
                    }
               }
               0x6000 => {
                    // 0x6XNN, set VX = NN
                    // println!("set register {} to {}", x, (opcode & 0x00FF));
                    let n: u8 = (opcode & 0x00FF) as u8;
                    self.vm.v[x] = n;
                    // // println!("x: {}, n: {}", x, n);
               }
               0x7000 => {
                    // 0x7XNN, add NN to VX
                    let n: u16 = 0x00FF & opcode;
                    // println!("add {} to  register {}", n, x);
                    let a: u8 = ((n + self.vm.v[x] as u16) & 0xFF) as u8;
                    self.vm.v[x] = a;
                    // println!("add {} to V{}", n, x);
               }
               0x8000 => {
                    // calls function eight() that handles 0x8XXX opcodes
                    self.eight();
               }
               0x9000 => {
                    // Skips the next instruction if VX doesn't equal VY.
                    let y: usize = ((0x00F0 & self.vm.opcode) >> 4) as usize;
                    if self.vm.v[x] != self.vm.v[y] {
                         self.vm.pc += 2;
                    }
               }
               0xA000 => {
                    // Sets I to the address NNN.
                    self.vm.i = (opcode as usize) & 0x0FFF;
               }
               0xB000 => {
                    // 0xBNNN Jumps to the address NNN plus V0.
                    self.vm.pc = (self.vm.v[0] as usize) + ((opcode as usize) & 0x0FFF);
               }
               0xC000 => {
                    // 0xCXNN, Sets VX to the result of a bitwise AND operation on a random number and NN
                    let n: u16 = rng.gen_range(0..256);
                    self.vm.v[x] = n as u8 & (0x00FF & opcode) as u8;
               }
               0xD000 => {
                    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N+1 pixels.
                    // Each row of 8 pixels is read as bit-coded starting from memory location I;
                    // I value doesn’t change after the execution of this instruction.
                    // As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
                    let y: usize = ((0x00F0 & opcode) >> 4) as usize;
                    let vx = self.vm.v[x] as usize;
                    let vy = self.vm.v[y] as usize;
                    let n = (0x000F as usize) & (opcode as usize);
                    if x == 4 && y == 5 {
                         // println!("x: {}, y: {}, n: {}, I: {}", vx, vy, n, self.vm.i);
                    }
                    self.vm.v[15] = 0;
                    for r in 0..n {
                         let start: usize = (r + vy) * WIDTH as usize + vx; // starting coordinate
                         let sprite: u8 = self.vm.memory[self.vm.i + r];
                         let mut s = format!("{:b}", sprite);
                         while s.len() < 8 {
                              let mut o = String::from("0");
                              o.push_str(s.as_str());
                              s = o;
                         }
                         let mut a = 0;
                         s.chars().for_each(|b| {
                              if (start + a) < (WIDTH * HEIGHT) as usize {
                                   let xo = b.to_string().parse::<u8>().unwrap()
                                        ^ self.vm.gfx[start + a];
                                   if xo != self.vm.gfx[start + a] {
                                        if xo == 0 {
                                             self.vm.v[15] = 1;
                                        }
                                        self.vm.gfx[start + a] = xo;
                                        self.update_canvas(
                                             (start + a) % WIDTH as usize,
                                             (start + a) / WIDTH as usize,
                                             self.vm.gfx[start + a],
                                        );
                                   }
                                   a += 1;
                              }
                         });
                    }
                    self.vm.canvas.present();
               }
               0xE000 => self.e(),
               0xF000 => self.f(),
               _ => {
                    // println!("invalid opcode");
               }
          }
     }
     fn f(&mut self) {
          let code = 0x00FF & self.vm.opcode;
          let x: usize = ((0x0F00 & self.vm.opcode) >> 8) as usize;
          match code {
               0x0007 => self.vm.v[x] = self.vm.delay_timer, // 0xFX07, Sets VX to the value of the delay timer.
               0x000A => {
                    // 0xFX0A, A key press is awaited, and then stored in VX.
                    while self.vm.keypress == 17 {
                         self.detect_keypress();
                    }
                    self.vm.v[x] = self.vm.keypress;
               }
               0x0015 => {
                    // 0xFX18, set delay timer to VX
                    self.vm.delay_timer = self.vm.v[x];
               }
               0x0018 => {
                    // 0xFX18, set sound timer to VX
                    self.vm.sound_timer = self.vm.v[x];
               }
               0x001E => {
                    // 0xFX1E, Adds VX to I. VF is not affected.
                    self.vm.i += self.vm.v[x] as usize;
               }
               0x0029 => {
                    // 0xFX29, Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                    let sa = 0x50 + self.vm.v[x] * 5;
                    self.vm.i = sa as usize;
                    // println!("Set I to {} from V{}", 0x50 + self.vm.v[x], x);
               }
               0x0033 => {
                    // Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
                    // println!("{:?}", self.vm.v);
                    let mut counter = 0;
                    let mut nstring: String = self.vm.v[x].to_string();
                    while nstring.len() < 3 {
                         let mut z = String::from("0");
                         z.push_str(nstring.as_str());
                         nstring = z;
                    }
                    nstring.chars().for_each(|n| {
                         self.vm.memory[self.vm.i + counter] = n.to_string().parse::<u8>().unwrap();
                         counter += 1;
                    });
                    // println!("storing {} at i addresses from V{}", nstring, x);
               }
               0x0055 => {
                    for z in 0..(x + 1) {
                         self.vm.memory[self.vm.i + z] = self.vm.v[z];
                    }
                    // println!("DUMP: store up to register V{} starting at {}",x, self.vm.i);
               }
               0x0065 => {
                    for z in 0..(x + 1) {
                         self.vm.v[z] = self.vm.memory[self.vm.i + z];
                    }
                    // println!("DUMP memory up to V{:x}", x);
               }
               _ => println!("not a valid opcode: {:x}", self.vm.opcode),
          }
     }
     fn e(&mut self) {
          let code = 0x00FF & self.vm.opcode;
          let x: usize = ((0x0F00 & self.vm.opcode) << 8) as usize;
          match code {
               0x009E => {
                    // EX9E, Skips the next instruction if the key stored in VX is pressed.
                    if self.vm.v[x] == self.vm.keypress {
                         self.vm.pc += 2;
                    }
               }
               0x00A1 => {
                    if self.vm.v[x] != self.vm.keypress {
                         self.vm.pc += 2;
                    }
               }
               _ => println!("e: {}", self.vm.opcode),
          }
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
                    self.vm.pc = (self.vm.stack[self.vm.sp]) as usize; // set next opcode to be the next opcode in memory after the prior call in the stack
               }
               _ => {
                    self.vm.pc = code as usize;
               }
          }
     }
     fn eight(&mut self) {
          // opcodes that begin with 8
          let code = 0x000F & self.vm.opcode;
          let x: usize = ((0x0F00 & self.vm.opcode) as usize) >> 8;
          let y: usize = ((0x00F0 & self.vm.opcode) as usize) >> 4;
          match code {
               // basic arithmetic and bitwise operations
               0x0000 => self.vm.v[x] = self.vm.v[y],
               0x0001 => self.vm.v[x] = self.vm.v[y] | self.vm.v[x],
               0x0002 => self.vm.v[x] = self.vm.v[y] & self.vm.v[x],
               0x0003 => self.vm.v[x] = self.vm.v[y] ^ self.vm.v[x],
               0x0004 => {
                    // 0x8XY4, Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                    // println!("add {} to {} from reg V{} into V{}",self.vm.v[x], self.vm.v[y], x, y);
                    let result: u16 = self.vm.v[x] as u16 + self.vm.v[y] as u16;
                    if (result & 0xFF) < result {
                         self.vm.v[15] = 1
                    } else {
                         self.vm.v[15] = 0;
                    }
                    self.vm.v[x] = (result & 0xFF) as u8;
               }
               0x0005 => {
                    // println!( "subtract {} from {} from reg V{} taking away V{}", self.vm.v[x], self.vm.v[y], x, y);
                    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                    if self.vm.v[x] < self.vm.v[y] {
                         // check if VX is < VY
                         let sub = self.vm.v[x] as i16 - self.vm.v[y] as i16 + 256;
                         self.vm.v[x] = sub as u8;
                         self.vm.v[0xF] = 0;
                    } else {
                         // if no, subtract normally and set VF to 1 to indicate no borrow
                         self.vm.v[x] -= self.vm.v[y];
                         self.vm.v[0xF] = 1;
                    }
               }
               0x0006 => {
                    self.vm.v[0xF] = self.vm.v[x] & 0x1;
                    self.vm.v[x] = self.vm.v[x] >> 1;
               }
               0x0007 => {
                    if self.vm.v[y] < self.vm.v[x] {
                         // 0x8XY7, check if VY is < VX
                         // if yes, shift VX to the left by one, subtract, then set VF to 0 to indicate a borrow
                         let sub = self.vm.v[y] as i16 - self.vm.v[x] as i16 + 256;
                         self.vm.v[x] = sub as u8;
                         self.vm.v[0xF] = 0;
                    } else {
                         // if no, subtract normally and set VF to 1 to indicate no borrow
                         self.vm.v[x] = self.vm.v[y] - self.vm.v[x];
                         self.vm.v[0xF] = 1;
                    }
               }
               0x000E => {
                    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                    let most_sig = self.vm.v[x] & 0b10000000;
                    self.vm.v[15] = most_sig;
                    self.vm.v[x] = self.vm.v[x] << 1;
               }
               _ => println!("invalid opcode"),
          }
     }
     fn clear_screen(&mut self) {
          self.vm.gfx = [0; (HEIGHT * WIDTH) as usize];
          println!("{:?}", self.vm.gfx);
          self.vm.canvas.clear();
          self.vm.canvas.present();
     }
     fn update_canvas(&mut self, x: usize, y: usize, b: u8) {
          self.vm
               .canvas
               .set_draw_color(Color::RGB(255 * b, 255 * b, 255 * b));
          let pixel = Rect::new(
               (x * PIXEL_SIZE as usize) as i32,
               (y * PIXEL_SIZE as usize) as i32,
               PIXEL_SIZE,
               PIXEL_SIZE,
          );
          self.vm.canvas.fill_rect(pixel).unwrap();
     }
     fn make_canvas(sdl_context: &sdl2::Sdl, title: &str) -> Canvas<Window> {
          let video_subsystem = sdl_context.video().unwrap();
          let window = video_subsystem
               .window(title, PIXEL_SIZE * WIDTH, PIXEL_SIZE * HEIGHT)
               .position(0, 0)
               .build()
               .unwrap();
          window.into_canvas().present_vsync().build().unwrap()
     }
     pub fn detect_keypress(&mut self) {
          self.vm.keypress = key_check::keycheck(&self.vm.sdl_context, self.vm.keypress);
     }
     pub fn graphics_init(&mut self) -> Result<(), String> {
          self.vm.canvas.present();
          Ok(())
     }
}
