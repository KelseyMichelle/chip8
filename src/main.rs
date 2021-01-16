use std::env;

mod chip8;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut emulator: chip8::Chip8 = chip8::Chip8::initialize();
    let game = args[1].as_str();
    let hrtz = args[2].parse::<u64>().unwrap();
    println!();
    emulator.load_game(game);
    emulator.graphics_init().unwrap();
    let sixty_hz = time::Duration::from_millis(1000 / hrtz);
    loop {
        thread::sleep(sixty_hz);
        emulator.detect_keypress();
        emulator.emulate_cycle();
    }
}
