use std::process;
use rust8080;

fn main() {
    let file = "./rom/spaceinvaders/invaders_final.rom";

    let mut memory = rust8080::load_rom(file)
    .unwrap_or_else(|err| {
        eprintln!("Error loading rom: {}", err);
        process::exit(-1);
    });

    let state = rust8080::State::new(&mut memory);

    rust8080::emulate(state);
}