mod chip8;
fn main() {
    let mut emulator: chip8::Chip8 = chip8::Chip8::initialize();
    emulator.load_game("pong");
    let mut c = 0;
    for i in 1..100 {
        c += emulator.emulate_cycle();
    }
    println!("{} unrecog opcodes", c);
}
