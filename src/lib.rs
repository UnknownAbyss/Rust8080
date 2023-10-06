use console_engine::rect_style::BorderStyle;
use console_engine::screen::Screen;
use console_engine::{Color, KeyCode, KeyModifiers, ConsoleEngine};
use emulator::arch::flag::{Flag, FlagType};
use emulator::arch::opcodes::Opcode;
pub use emulator::arch::state::State;
use emulator::iset::run_op;
use std::{process, vec};
use std::{error::Error, fs};

use crate::emulator::utils::join_bytes;
mod emulator;
use crate::constants::*;
mod constants;


pub fn load_rom(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_data = fs::read(file_path)?;
    Ok(file_data)
}

pub fn emulate(mut state: State) {
    let mut engine =
        console_engine::ConsoleEngine::init(constants::WIDTH, HEIGHT, TARGET_FPS).unwrap_or_else(|err| {
            println!("Could not create screen: {err}");
            process::exit(-1);
        });
    
    let mut show_keybinds = true;

    loop {
        engine.wait_frame();
        engine.check_resize();
        engine.clear_screen();

        engine.rect_border(
            1,
            1,
            engine.get_width() as i32 - 2,
            engine.get_height() as i32 - 2,
            BorderStyle::new_double().with_colors(DULL, Color::Reset),
        );

        engine.print_screen(3, 2, &disass(&state, engine.get_height(), engine.get_width()));
        engine.print_screen(engine.get_width() as i32 - 23, 2, &keybinds(&show_keybinds));
        engine.print_fbg(engine.get_width() as i32 - 19, engine.get_height() as i32 - 3, "keybinds: (h)elp", DARKENAB, Color::Reset);


        if process_input(&engine, &mut state, &mut show_keybinds) {
            break;
        }
        engine.draw();
    }
}

fn process_input(engine: &ConsoleEngine, state: &mut State, kb: &mut bool ) -> bool {
    if engine.is_key_pressed(KeyCode::Char('q'))
        || engine.is_key_pressed(KeyCode::Char('Q'))
        || engine.is_key_pressed(KeyCode::Esc)
        || engine.is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL, console_engine::KeyEventKind::Press)
        || engine.is_key_pressed_with_modifier(KeyCode::Char('C'), KeyModifiers::CONTROL, console_engine::KeyEventKind::Press)
    {
        return true;
    }

    if engine.is_key_pressed(KeyCode::Char('r'))
        || engine.is_key_pressed(KeyCode::Char('R')) {
        run_op(state);
    }

    if engine.is_key_pressed(KeyCode::Char('h'))
        || engine.is_key_pressed(KeyCode::Char('H'))
        || engine.is_key_pressed(KeyCode::Tab) {
        *kb = !*kb;
    }

    false
}

fn keybinds(kb: &bool) -> Screen {
    let kbinds = vec![
        "r   - run once",
        "q   - quit",
        "tab - keybinds"
    ];
    
    let mut scr = Screen::new(20, kbinds.len() as u32 + 2);

    if *kb {
        scr.rect_border(
            0,
            0,
            scr.get_width() as i32 - 1,
            scr.get_height() as i32 - 1,
            BorderStyle::new_double().with_colors(DARK, Color::Reset),
        );
    
        for i in kbinds.iter().enumerate() {
            scr.print_fbg(2, i.0 as i32 + 1  , i.1, ENABLED, Color::Reset);
        }
    }

    scr
}

fn disass(state: &State, height: u32, width: u32) -> Screen {
    let mut scr = Screen::new(width - 6, height - 4);

    scr.print_screen(1, 0, &display_status(state));
    scr.print_screen(1, 6, &display_ops(state, height));

    scr
}

fn display_status(state: &State) -> Screen {
    let mut scr = Screen::new(55, 5);

    scr.print_fbg(1, 0, "Flags:", NORMAL, Color::Reset);
    scr.print_screen(0, 1, &display_flags(&state.flags));

    scr.print_fbg(22, 0, "Registers:", NORMAL, Color::Reset);
    scr.print_screen(21, 1, &display_regs(&state));
    scr
}

fn display_regs(state: &State) -> Screen {
    let mut scr = Screen::new(32, 4);
    scr.rect_border(
        0,
        0,
        scr.get_width() as i32 - 2,
        scr.get_height() as i32 - 1,
        BorderStyle::new_heavy().with_colors(DARK, Color::Reset),
    );
    scr.print_fbg(2, 1, "Ac  BC   DE   HL   PC   SP", NORMAL, Color::Reset);
    scr.print_fbg(
        2,
        2,
        format!("{:02x}", state.a,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(
        5,
        2,
        format!("{:02x}", state.b,).as_str(),
        DARKENAB,
        Color::Reset,
    );
    scr.print_fbg(
        7,
        2,
        format!("{:02x}", state.c,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(
        10,
        2,
        format!("{:02x}", state.d,).as_str(),
        DARKENAB,
        Color::Reset,
    );
    scr.print_fbg(
        12,
        2,
        format!("{:02x}", state.e,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(
        15,
        2,
        format!("{:02x}", state.h,).as_str(),
        DARKENAB,
        Color::Reset,
    );
    scr.print_fbg(
        17,
        2,
        format!("{:02x}", state.l,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(
        20,
        2,
        format!("{:04x}", state.pc,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(
        25,
        2,
        format!("{:04x}", state.sp,).as_str(),
        ENABLED,
        Color::Reset,
    );
    scr
}

fn display_flags(flags: &Flag) -> Screen {
    let mut scr = Screen::new(19, 4);
    scr.rect_border(
        0,
        0,
        scr.get_width() as i32 - 1,
        scr.get_height() as i32 - 1,
        BorderStyle::new_heavy().with_colors(DARK, Color::Reset),
    );
    scr.print_fbg(2, 1, "Z S P C A", NORMAL, Color::Reset);
    scr.print_fbg(
        2,
        2,
        format!(
            "{} {} {} {} {}",
            flags.get(FlagType::Z),
            flags.get(FlagType::S),
            flags.get(FlagType::P),
            flags.get(FlagType::CY),
            flags.get(FlagType::AC),
        )
        .as_str(),
        ENABLED,
        Color::Reset,
    );
    scr.print_fbg(12, 1, "P A D", DULL, Color::Reset);
    scr.print_fbg(
        12,
        2,
        format!(
            "{} {} {}",
            flags.get(FlagType::PAD) >> 2 & 0b1,
            flags.get(FlagType::PAD) >> 1 & 0b1,
            flags.get(FlagType::PAD) & 0b1,
        )
        .as_str(),
        DISABLED,
        Color::Reset,
    );

    scr
}

fn display_ops(state: &State, height: u32) -> Screen {
    let mut scr = Screen::new(60, height - 11);
    let mut counter = 0 as i32;
    let mut ip = state.pc as usize;

    scr.rect_border(
        0,
        0,
        scr.get_width() as i32 - 1,
        scr.get_height() as i32 - 1,
        BorderStyle::new_heavy().with_colors(DARK, Color::Reset),
    );

    scr.print_fbg(
        2,
        0,
        "  Address  Opcode   Intruction  ",
        NORMAL,
        Color::Reset,
    );

    while ip < state.mem.len() && counter < (height / 2 - 7) as i32 {
        let opcode = state.mem[ip];

        if counter == 0 {
            scr.print_fbg(2, 2, ">", NORMAL, Color::Reset);
        }

        let instr_addr = format!("{:#06x}:", ip).to_string();
        let instr = format!("{:02}", Opcode::convert(opcode)).to_string();
        let opc = format!("{:02x}", opcode).to_string();

        scr.print_fbg(4, 2 * (counter + 1), &instr_addr, DARK, Color::Reset);
        scr.print_fbg(13, 2 * (counter + 1), &opc, ENABLED, Color::Reset);
        scr.print_fbg(22, 2 * (counter + 1), &instr, HIGHLIGHT, Color::Reset);

        match Opcode::cycles(opcode) {
            3 => {
                let adr = join_bytes(state.mem[ip + 2], state.mem[ip + 1]);
                let adr = format!("{:04x}", adr).to_string();
                scr.print_fbg(33, 2 * (counter + 1), &adr, LIGHT, Color::Reset);

                let r_adr =
                    format!("{:02x}{:02x}", state.mem[ip + 1], state.mem[ip + 2]).to_string();
                scr.print_fbg(15, 2 * (counter + 1), &r_adr, DARKENAB, Color::Reset);
                ip += 3;
            }
            2 => {
                let adr = state.mem[ip + 1];
                let adr = format!("{:02x}", adr).to_string();
                scr.print_fbg(33, 2 * (counter + 1), &adr, LIGHT, Color::Reset);

                let r_adr = format!("{:02x}", state.mem[ip + 1]).to_string();
                scr.print_fbg(15, 2 * (counter + 1), &r_adr, DARKENAB, Color::Reset);
                ip += 2;
            }
            _ => {
                ip += 1;
            }
        }

        counter += 1;
    }
    scr
}
