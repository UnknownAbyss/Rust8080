use macroquad::window::Conf;
use rust8080;
use std::process;

fn window_conf() -> Conf {
    Conf {
        window_title: "8080 Emulator".to_owned(),
        window_width: 560,
        window_height: 640,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let file = "./rom/spaceinvaders/space-invaders.rom";

    let memory = rust8080::load_rom(file).unwrap_or_else(|err| {
        eprintln!("Error loading rom: {}", err);
        process::exit(-1);
    });

    let state = rust8080::State::new(memory);
    let io = rust8080::IO::new(); 
    rust8080::emulate(state, io).await;
}
