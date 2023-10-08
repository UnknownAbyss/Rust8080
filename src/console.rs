use console_engine::{
    pixel, rect_style::BorderStyle, screen::Screen, Color, ConsoleEngine, KeyCode, KeyModifiers,
};

pub use crate::{
    emulator::{
        arch::{
            flag::{Flag, FlagType},
            opcodes::Opcode,
        },
        iset::run_op,
        utils::join_bytes,
    },
    State,
};

use crate::video::graphics;


pub const WIDTH: u32 = 50;
pub const HEIGHT: u32 = 20;
pub const TARGET_FPS: u32 = 90;

pub const DARK: Color = Color::Rgb {
    r: 148,
    g: 93,
    b: 6,
};
pub const NORMAL: Color = Color::Rgb {
    r: 247,
    g: 165,
    b: 0,
};
pub const LIGHT: Color = Color::Rgb {
    r: 255,
    g: 211,
    b: 122,
};
pub const DULL: Color = Color::Rgb {
    r: 84,
    g: 66,
    b: 29,
};
pub const DISABLED: Color = Color::Rgb {
    r: 87,
    g: 87,
    b: 87,
};
pub const DARKENAB: Color = Color::Rgb {
    r: 117,
    g: 117,
    b: 117,
};
pub const ENABLED: Color = Color::Rgb {
    r: 187,
    g: 187,
    b: 187,
};
pub const HIGHLIGHT: Color = Color::Rgb {
    r: 230,
    g: 80,
    b: 0,
};

pub async fn process_input(
    engine: &ConsoleEngine,
    state: &mut State,
    kb: &mut bool,
    mv: &mut i32,
    debug: &mut bool,
    live: &mut bool,
) -> bool {
    if engine.is_key_pressed(KeyCode::Char('q'))
        || engine.is_key_pressed(KeyCode::Char('Q'))
        || engine.is_key_pressed(KeyCode::Esc)
        || engine.is_key_pressed_with_modifier(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
            console_engine::KeyEventKind::Press,
        )
        || engine.is_key_pressed_with_modifier(
            KeyCode::Char('C'),
            KeyModifiers::CONTROL,
            console_engine::KeyEventKind::Press,
        )
    {
        return true;
    }

    if engine.is_key_held(KeyCode::Char('r')) {
        *mv = 0;
        run_op(state);
        if *live {
            graphics::graphics(&state.mem).await;
        }
    }
    
    if engine.is_key_held(KeyCode::Char('R')) {
        *mv = 0;
        for _ in 0..100 {
            run_op(state);
        }
        if *live {
            graphics::graphics(&state.mem).await;
        }
    }

    if engine.is_key_held(KeyCode::Char('d')) || engine.is_key_pressed(KeyCode::Char('D')) {
        *debug = !*debug;
    }
    
    if engine.is_key_held(KeyCode::Char('l')) || engine.is_key_pressed(KeyCode::Char('L')) {
        *live = !*live;
    }

    if engine.is_key_held(KeyCode::Char('s')) || engine.is_key_pressed(KeyCode::Char('S')) {
        graphics::graphics(&state.mem).await;
    }

    if engine.is_key_pressed(KeyCode::Char('h'))
        || engine.is_key_pressed(KeyCode::Char('H'))
        || engine.is_key_pressed(KeyCode::Char('k'))
        || engine.is_key_pressed(KeyCode::Char('K'))
        || engine.is_key_pressed(KeyCode::Tab)
        || engine.is_key_pressed(KeyCode::Char('?'))
    {
        *kb = !*kb;
    }

    if engine.is_key_held(KeyCode::Up) && *mv + state.pc as i32 > 0 {
        *mv = *mv - 1;
    }
    if engine.is_key_held(KeyCode::Down) {
        *mv = *mv + Opcode::cycles(state.mem[(state.pc as i32 + *mv) as usize]) as i32;
    }
    if engine.is_key_pressed(KeyCode::Char(' ')) {
        *mv = 0;
    }

    false
}

pub fn keybinds() -> Screen {
    let kbinds = [
        "r     - run once",
        "R     - run 100",
        "up    - scroll up",
        "down  - scroll down",
        "space - reset scroll",
        "d     - toggle debug",
        "L     - toggle live",
        "s     - sync screen",
        "q     - quit",
        "tab   - keybinds",
    ];

    let mut scr = Screen::new(24, kbinds.len() as u32 + 2);

    scr.rect_border(
        0,
        0,
        scr.get_width() as i32 - 1,
        scr.get_height() as i32 - 1,
        BorderStyle::new_double().with_colors(DARK, Color::Reset),
    );

    for i in kbinds.iter().enumerate() {
        scr.print_fbg(2, i.0 as i32 + 1, i.1, ENABLED, Color::Reset);
    }

    scr
}

pub fn disass(state: &State, height: u32, width: u32, line: &i32) -> Screen {
    let mut scr = Screen::new(width - 6, height - 4);

    scr.print_screen(1, 0, &display_status(state));
    scr.print_screen(1, 6, &display_ops(state, height, line));

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

fn display_ops(state: &State, height: u32, line: &i32) -> Screen {
    let mut scr = Screen::new(60, height - 11);
    let mut counter = 0 as i32;
    let mut ip = state.pc as i32 + line;

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

    if *line != 0 {
        scr.rect_border(
            2,
            2,
            scr.get_width() as i32 - 3,
            2,
            BorderStyle::new_simple().with_colors(DISABLED, Color::Reset),
        );
        let lineno = format!(
            " {:#06x}{}{}] ",
            state.pc as i32,
            if *line >= 0 { " [+" } else { " [" },
            line
        )
        .to_string();
        scr.print_fbg(
            scr.get_width() as i32 - 4 - lineno.len() as i32,
            2,
            &lineno,
            ENABLED,
            Color::Reset,
        );
    } else {
        scr.print_fbg(2, 2, ">", NORMAL, Color::Reset);
    }

    while ip < (state.mem.len() as i32) && counter < (height / 2 - 7) as i32 {
        if ip < 0 {
            counter += 1;
            ip += 1;
            continue;
        }

        let _ip = ip as usize;
        let opcode = state.mem[_ip];

        let instr_addr = format!("{:#06x}:", _ip).to_string();
        let instr = format!("{:02}", Opcode::convert(opcode)).to_string();
        let opc = format!("{:02x}", opcode).to_string();

        scr.print_fbg(4, 2 * (counter + 1), &instr_addr, DARK, Color::Reset);
        scr.print_fbg(13, 2 * (counter + 1), &opc, ENABLED, Color::Reset);
        scr.print_fbg(22, 2 * (counter + 1), &instr, HIGHLIGHT, Color::Reset);

        match Opcode::cycles(opcode) {
            3 => {
                let adr = join_bytes(state.mem[_ip + 2], state.mem[_ip + 1]);
                let adr = format!("{:04x}", adr).to_string();
                scr.print_fbg(33, 2 * (counter + 1), &adr, LIGHT, Color::Reset);

                let r_adr =
                    format!("{:02x}{:02x}", state.mem[_ip + 1], state.mem[_ip + 2]).to_string();
                scr.print_fbg(15, 2 * (counter + 1), &r_adr, DARKENAB, Color::Reset);
                ip += 3;
            }
            2 => {
                let adr = state.mem[_ip + 1];
                let adr = format!("{:02x}", adr).to_string();
                scr.print_fbg(33, 2 * (counter + 1), &adr, LIGHT, Color::Reset);

                let r_adr = format!("{:02x}", state.mem[_ip + 1]).to_string();
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

pub fn pulse_anim(pulse: &f32) -> Screen {
    let mut scr = Screen::new(13, 13);

    match *pulse as u32 % 6 {
        0 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                1,
                pixel::pxl_fg('@', DARK),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                2,
                pixel::pxl_fg('O', HIGHLIGHT),
            );
        }
        1 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                2,
                pixel::pxl_fg('O', HIGHLIGHT),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                3,
                pixel::pxl_fg('o', NORMAL),
            );
        }
        2 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                3,
                pixel::pxl_fg('o', NORMAL),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                4,
                pixel::pxl_fg('*', DARKENAB),
            );
        }
        3 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                4,
                pixel::pxl_fg('*', DARKENAB),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                5,
                pixel::pxl_fg('.', DISABLED),
            );
        }
        4 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                5,
                pixel::pxl_fg('.', DISABLED),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                6,
                pixel::pxl_fg('.', DULL),
            );
        }
        5 => {
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                6,
                pixel::pxl_fg('.', DULL),
            );
            scr.circle(
                scr.get_width() as i32 / 2,
                scr.get_height() as i32 / 2,
                1,
                pixel::pxl_fg('@', DARK),
            );
        }
        _ => (),
    }

    scr
}
