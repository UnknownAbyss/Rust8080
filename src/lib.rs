use console_engine::rect_style::BorderStyle;
use console_engine::Color;
use console_engine::{self, pixel, ConsoleEngine};
pub use emulator::arch::state::State;
use machine::video::graphics;
pub use machine::io::IO;
use std::env::Args;
use std::process;
use std::{error::Error, fs};
use console::*;

mod emulator;
mod console;
mod machine;

pub fn get_file(cmd_line: Args) -> Option<String> {
    let mut cmds = cmd_line.into_iter();
    cmds.next();
    match cmds.next() {
        Some(x) => Some(x),
        None => None
    }
}

pub fn load_rom(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut memory = vec![0; 0x10000];
    let file_data = fs::read(file_path)?;

    let mut i = 0;
    for byte in file_data {
        memory[i] = byte;
        i += 1;
    }
    Ok(memory)
}

pub async fn emulate(mut state: State, mut io: IO) {
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
    let mut int = true;

    loop {
        engine.wait_frame();
        engine.check_resize();
        engine.clear_screen();
        
        if process_input(&engine, &mut state, &mut io, &mut show_keybinds, &mut mv, &mut debug, &mut live).await {
            break;
        };

        if debug {
            debug_state(&mut engine, &state, &io, &mut pulse, &show_keybinds, &mv);
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
            running_state(&mut engine, &mut state, &mut io, &mut debug);
            engine.print(0, 0, &format!("{}", engine.frame_count));
            if state.enable == 1 && !debug{
                if int {
                    state.generate_interrupt(1);
                } else {
                    state.generate_interrupt(2);
                }
                int = !int;
            }
            if graphics::graphics(&state.mem, &mut io).await {
                debug = true;
            }
        }


        engine.draw();
    }
}

fn debug_state(
    engine: &mut ConsoleEngine,
    state: &State,
    io: &IO,
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
        &disass(&state, &io, engine.get_height(), engine.get_width(), &mv),
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

fn running_state(engine: &mut ConsoleEngine, state: &mut State, io: &mut IO, _debug: &mut bool) {
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

    engine.print_screen(3, 3, &display_ports(&io));

    for _ in 0..5000 {
        state.run_op(io);

        // Breakpoints
        // if state.pc == 0x0bf4 {
        //     *debug = !*debug;
        //     break;
        // }
    }
}