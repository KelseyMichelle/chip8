# chip8 emulator

Uses command line interfacing to load emulators

Syntax:

```cargo run [ROM_FILENAME] [CLOCK_SPEED]```

For clock speed, I recommend starting at 300hz then deciding, from there, whether you would like to increase or decrease it.

Example Controls:
Pong: A and Z for left side, and M and K for right side.
Each game will map controls differently.

Put roms into the ./src/roms/ directory

**Technologies Used**
- Rust Programming Language
- SDL2 Graphics Library
