use macroquad::window::Conf;
use rust8080;
use std::process;
mod video;


fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        window_width: 560,
        window_height: 640,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let file = "./rom/spaceinvaders/invaders_final.rom";

    let memory = rust8080::load_rom(file).unwrap_or_else(|err| {
        eprintln!("Error loading rom: {}", err);
        process::exit(-1);
    });

    let mut state = rust8080::State::new(memory);

    rust8080::emulate(&mut state).await;
    
}
