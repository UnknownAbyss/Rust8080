use std::process;
use macroquad::prelude::*;
use crate::{IO, machine::io::Actions};

pub async fn graphics(mem: &Vec<u8>, io: &mut IO) -> bool {
    clear_background(BLACK);

    handle_input(io);

    if is_quit_requested() { process::exit(-1) }

    draw_screen(&mem);
    next_frame().await;

    if is_key_down(KeyCode::D) { return true }
    false
}

fn draw_screen(mem: &Vec<u8>) {
    let w = screen_width();
    let h = screen_height();
    let facx = w/224.0;
    let facy = h/256.0;

    for j in 0..224 {
        for i in 0..32 {
            let byte = mem[0x2400 + (31 - i) + j*32];
            
            for bit in 0..8 {
                let val = ((byte << bit) >> 7) & 0b1 == 0b1;
                
                if val {
                    // x and y swapped as video buffer is transposed
                    draw_rectangle( 
                        facy* (j as f32), 
                        facx*((i*8 + bit) as f32), 
                        facy, 
                        facx, 
                        GREEN
                    );
                }
            }
        }
    }
    
    draw_text("(d)ebug", 1.0, w / 20.0, w / 20.0, RED);
}

fn handle_input(io: &mut IO) {
    io.io_reset_input();

    if is_key_down(KeyCode::Comma) {
        io.io_op(Actions::P1Left);
    }
    if is_key_down(KeyCode::Period) {
        io.io_op(Actions::P1Right);
    }
    if is_key_down(KeyCode::Slash) {
        io.io_op(Actions::P1Shot);
    }

    if is_key_down(KeyCode::Q) {
        io.io_op(Actions::P2Left);
    }
    if is_key_down(KeyCode::W) {
        io.io_op(Actions::P2Right);
    }
    if is_key_down(KeyCode::E) {
        io.io_op(Actions::P2Shot);
    }

    if is_key_down(KeyCode::Key8) {
        io.io_op(Actions::Credit);
    }
    if is_key_down(KeyCode::Key9) {
        io.io_op(Actions::P1Start);
    }
    if is_key_down(KeyCode::Key0) {
        io.io_op(Actions::P2Start);
    }
}