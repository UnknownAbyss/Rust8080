use std::io::{stdout, Write};
use std::{io, process};
use std::{fs, error::Error};
pub use emulator::arch::state::State;
use emulator::arch::opcodes::Opcode;
use emulator::iset::run_op;

use crate::emulator::utils::join_bytes;
mod emulator;


pub fn load_rom(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_data = fs::read(file_path)?;
    Ok(file_data)
}

pub fn emulate(mut state: State) {
    disass(&state);

    loop {
        match getinp() {
            Inp::Run => {
                run_op(&mut state);
                disass(&state);
            },
            Inp::Exit => break,
            Inp::Unk => println!("Unknown Command")
        };
    }
}

fn disass(state: &State) {
    println!("\n------------------------------------------------------");
    display_status(state);
    display_ops(state, 10);
}

fn display_status(state: &State) {
    println!("\nFlag: ZSPCApad");
    println!("      {:08b}", state.flags.reg);
    println!("Reg:  a  b  c  d  e  h  l   sp   pc");
    println!(
        "      {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:04x} {:04x}", 
        state.a, 
        state.b, 
        state.c, 
        state.d, 
        state.e, 
        state.h, 
        state.l, 
        state.sp, 
        state.pc
    );
}

fn display_ops(state: &State, n: u8) {
    println!("\n");
    
    let mut i = 0;
    let mut c = 0;
    let mut mem_adr = state.pc as usize;
    
    while mem_adr < state.mem.len() && c < n {
        mem_adr = (state.pc + i) as usize;
        let opcode = state.mem[mem_adr];
        
        match Opcode::cycles(Opcode::convert(opcode)) {
            3 => {
                let adr = join_bytes(
                    state.mem[mem_adr + 2],
                    state.mem[mem_adr + 1]
                );
                println!(
                    "{:04x}: {:02x} {} {:04x}", 
                    mem_adr, 
                    opcode, 
                    Opcode::convert(opcode), 
                    adr
                );
                i += 2;
            },
            2 => {
                println!(
                    "{:04x}: {:02x} {} {:02x}", 
                    mem_adr, 
                    opcode, 
                    Opcode::convert(opcode), 
                    state.mem[mem_adr + 1]
                );
                i += 1;
            },
            _ => println!(
                "{:04x}: {:02x} {}", 
                mem_adr, 
                opcode, 
                Opcode::convert(opcode)
            )
        }
        c += 1;
        i += 1;
    }
}

enum Inp {
    Run,
    Exit,
    Unk,
}

fn getinp() -> Inp {
    let mut inp = String::new();
    
    print!("> ");
    stdout().flush().unwrap_or_else(|err| {
        println!("Could not flush stdout: {err}");
        process::exit(-1);
    });

    io::stdin().read_line(&mut inp).unwrap_or_else(|err| {
        println!("Could not read character: {err}");
        process::exit(-1);
    });

    match &inp.trim()[..] {
        "r" => Inp::Run,
        "q" => Inp::Exit,
        _ => Inp::Unk,
    }
}