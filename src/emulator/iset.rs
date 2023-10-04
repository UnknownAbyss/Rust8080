use std::process;
use super::utils::*;
use super::arch::{state::State, opcodes::Opcode};


pub fn run_op(state: &mut State) {
    let _pc = state.pc as usize;
    let opcode = state.mem[_pc];


    match Opcode::convert(opcode) {
        Opcode::NOP => (),
        Opcode::LXIB => {
            state.b = state.mem[_pc + 2];
            state.c = state.mem[_pc + 1];
            state.pc += 2;
        },
        Opcode::STAXB => {
            let bc = join_bytes(state.b, state.c);
            state.mem[bc as usize] = state.a;
        },
        Opcode::INXB => (state.b, state.c) = split_bytes(join_bytes(state.b, state.c) + 1),
        Opcode::INRB => {
            // check_flag_ac(state.b, state.b + 1, state);
            state.b += 1;
            check_flag_z(state.b, state);
            check_flag_s(state.b, state);
            check_flag_p(state.b, state);
        },
        Opcode::DCRB => {
            // check_flag_ac(state.b, state.b - 1, state);
            state.b -= 1;
            check_flag_z(state.b, state);
            check_flag_s(state.b, state);
            check_flag_p(state.b, state);

        },
        Opcode::MVIB => {
            state.b = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::RLC => {
            check_flag_cy((state.b as u16) << 1, state);
            state.b = state.b.rotate_left(1);
        },
        Opcode::NIMP(x) => {
            println!("Instruction {:#04x} not implemented", x);
            process::exit(-1);  
        },
    };


    state.pc += 1;
}