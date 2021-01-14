mod chip8;
fn main() {
    let mut emulator: chip8::Chip8 = chip8::Chip8::initialize();
    emulator.load_game("pong");
}
