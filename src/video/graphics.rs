use std::process;

use macroquad::prelude::*;

pub async fn graphics(mem: &Vec<u8>) -> bool {
    clear_background(BLACK);
    let w = screen_width();
    let h = screen_height();
    let facx = w/224.0;
    let facy = h/256.0;

    if is_quit_requested() {
        process::exit(-1);
    }
    
    for j in 0..224 {
        for i in 0..32 {
            let byte = mem[0x2400 + i + j*32];
            
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

    next_frame().await;
    
    if is_key_down(KeyCode::D) { return true }

    false
}