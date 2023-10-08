use console_engine::rect_style::BorderStyle;
use console_engine::Color;
use console_engine::{self, pixel, ConsoleEngine};
pub use emulator::arch::state::State;
use emulator::iset::run_op;
use video::graphics;
use std::process;
use std::{error::Error, fs};
use console::*;

mod emulator;
mod console;
mod video;

pub fn load_rom(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_data = fs::read(file_path)?;
    Ok(file_data)
}

pub async fn emulate(mut state: &mut State) {
    let mut engine =
        console_engine::ConsoleEngine::init(WIDTH, HEIGHT, TARGET_FPS).unwrap_or_else(|err| {
            println!("Could not create screen: {err}");
            process::exit(-1);
        });

    let mut show_keybinds = false;
    let mut mv = 0;
    let mut pulse = 0.0;
    let mut debug = true;
    let mut live = false;

    loop {
        engine.wait_frame();
        engine.check_resize();
        engine.clear_screen();
        
        if process_input(&engine, &mut state, &mut show_keybinds, &mut mv, &mut debug, &mut live).await {
            break;
        };

        if debug {
            debug_state(&mut engine, &state, &mut pulse, &show_keybinds, &mv);
            if live {
                engine.print_fbg(
                    engine.get_width() as i32 - 33,
                    engine.get_height() as i32 - 3,
                    "LIVE",
                    HIGHLIGHT,
                    Color::Reset,
                );
            }
        } else {
            running_state(&mut engine, &mut state);
            engine.print(0, 0, &format!("{}", engine.frame_count));
            debug = graphics::graphics(&state.mem).await;
        }


        engine.draw();
    }
}

fn debug_state(
    engine: &mut ConsoleEngine,
    state: &State,
    pulse: &mut f32,
    show_keybinds: &bool,
    mv: &i32,
) {
    engine.rect_border(
        1,
        1,
        engine.get_width() as i32 - 2,
        engine.get_height() as i32 - 2,
        BorderStyle::new_double().with_colors(DULL, Color::Reset),
    );

    engine.print_screen(
        3,
        2,
        &disass(&state, engine.get_height(), engine.get_width(), &mv),
    );

    engine.print_screen(79, engine.get_height() as i32 / 2 - 6, &pulse_anim(pulse));
    *pulse += 0.1;

    if *show_keybinds {
        engine.print_screen(engine.get_width() as i32 - 26, 2, &keybinds());
    }

    engine.print_fbg(
        engine.get_width() as i32 - 19,
        engine.get_height() as i32 - 3,
        "keybinds: (h)elp",
        DARKENAB,
        Color::Reset,
    );
}

fn running_state(engine: &mut ConsoleEngine, state: &mut State) {
    engine.fill_rect(
        1,
        1,
        engine.get_width() as i32 - 2,
        engine.get_height() as i32 - 2,
        pixel::pxl_bg(' ', DISABLED),
    );
    engine.rect_border(
        1,
        1,
        engine.get_width() as i32 - 2,
        engine.get_height() as i32 - 2,
        BorderStyle::new_double().with_colors(NORMAL, Color::Reset),
    );
    engine.fill_rect(
        engine.get_width() as i32 / 2 - 20,
        engine.get_height() as i32 / 2 - 2,
        engine.get_width() as i32 / 2 + 18,
        engine.get_height() as i32 / 2 + 2,
        pixel::pxl_bg(' ', Color::Reset),
    );
    engine.rect_border(
        engine.get_width() as i32 / 2 - 20,
        engine.get_height() as i32 / 2 - 2,
        engine.get_width() as i32 / 2 + 18,
        engine.get_height() as i32 / 2 + 2,
        BorderStyle::new_double().with_colors(NORMAL, Color::Reset),
    );
    engine.print_fbg(
        engine.get_width() as i32 / 2 - 18,
        engine.get_height() as i32 / 2,
        "Running - (d)ebug on game to return",
        NORMAL,
        Color::Reset,
    );

    for _ in 0..50 {
        run_op(state);
    }
}