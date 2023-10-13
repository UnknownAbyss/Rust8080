# Rust 8080 

An intel 8080 processor emulated in rust.

![](https://user-images.githubusercontent.com/44570898/274915867-c94feeeb-9f54-4c97-80c1-ab44c5382965.png)

## Features
- TUI Disassembler with port tracking
- Descriptive opcode display and navigation
- graphical display mapped to memory 0x2400 - 0x4000

## Installation
```sh
git clone https://github.com/UnknownAbyss/Rust8080.git
cd Rust8080/
cargo build --release
```

## Running
Command -
```bash
./target/release/rust8080 <rom_path>
```
Example space invaders rom -
```bash
./target/release/rust8080
```
or
```bash
./target/release/rust8080 rom/spaceinvaders/space-invaders.rom
```

## Usage
TUI keybinds can be seen by pressing h for (h)elp

For the example rom space-invaders, controls are mapped as follows (**For Graphical Screen**)

| Button    | Action            |
| ---       | ---               |
| 8 (eight) | Deposit credit    |
| 9 (nine)  | Player 1 Start    |
| , (comma) | Player 1 left     |
| . (period)| Player 1 right    |
| / (slash) | Player 1 fire     |
| 0 (zero)  | Player 2 Start    |
| Q         | Player 2 left     |
| W         | Player 2 right    |
| E         | Player 2 fire     |
| **d**     | **Switch debug**  |

## Build your own!

These are my main resources for the entire project

- [Emulator101](http://emulator101.com/)
- [Computer Archeology - Space invaders data](http://computerarcheology.com/Arcade/SpaceInvaders/)

> Thank you!