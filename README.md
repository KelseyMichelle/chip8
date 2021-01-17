# chip8 emulator

uses command line interfacing to load emulators

syntax:

```cargo run [ROM_FILENAME] [CLOCK_SPEED]```

for clock speed i recommend starting at 300hz then deciding, from there, whether you would like to increase or decrease it.

controls for pong are A and Z for left side, and M and K for right side.
controls are difficult because each game/rom interprets them differently.

put roms into the ./src/roms/ directory
