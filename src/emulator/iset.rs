use std::process;
use super::utils::*;
use super::arch::{state::State, opcodes::Opcode, flag::FlagType};


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
            check_flag_cy8((state.a as u16) << 1, state);
            state.a = state.a.rotate_left(1);
        },
        Opcode::DADB => {
            let hl = join_bytes(state.h, state.l);
            let bc = join_bytes(state.b, state.c);
            check_flag_cy16( hl as u32 + bc as u32, state);
            (state.h, state.l) = split_bytes(hl + bc);
        },
        Opcode::LDAXB => {
            let bc = join_bytes(state.b, state.c);
            state.a = state.mem[bc as usize];
        },
        Opcode::DCXB => (state.b, state.c) = split_bytes(join_bytes(state.b, state.c) - 1),
        Opcode::INRC => {
            // check_flag_ac(state.c, state.c + 1, state);
            state.c += 1;
            check_flag_z(state.c, state);
            check_flag_s(state.c, state);
            check_flag_p(state.c, state);
        },
        Opcode::DCRC => {
            // check_flag_ac(state.c, state.c - 1, state);
            state.c -= 1;
            check_flag_z(state.c, state);
            check_flag_s(state.c, state);
            check_flag_p(state.c, state);
        },
        Opcode::MVIC => {
            state.c = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::RRC => {
            check_flag_cy8((state.a as u16).rotate_right(1), state);
            state.a = state.a.rotate_right(1);
        },
        Opcode::LXID => {
            state.d = state.mem[_pc + 2];
            state.e = state.mem[_pc + 1];
            state.pc += 2;
        },
        Opcode::STAXD => {
            let de = join_bytes(state.d, state.e);
            state.mem[de as usize] = state.a;
        },
        Opcode::INXD => (state.d, state.e) = split_bytes(join_bytes(state.d, state.e) + 1),
        Opcode::INRD => {
            // check_flag_ac(state.b, state.b + 1, state);
            state.d += 1;
            check_flag_z(state.d, state);
            check_flag_s(state.d, state);
            check_flag_p(state.d, state);
        },
        Opcode::DCRD => {
            // check_flag_ac(state.b, state.b - 1, state);
            state.d -= 1;
            check_flag_z(state.d, state);
            check_flag_s(state.d, state);
            check_flag_p(state.d, state);

        },
        Opcode::MVID => {
            state.d = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::RAL => {
            let prev = state.flags.get(FlagType::CY);
            check_flag_cy8((state.a as u16) << 1, state);
            state.a = state.a << 1;
            state.a |= prev;
        },
        Opcode::DADD => {
            let hl = join_bytes(state.h, state.l);
            let de = join_bytes(state.d, state.e);
            check_flag_cy16( hl as u32 + de as u32, state);
            (state.h, state.l) = split_bytes(hl + de);
        },
        Opcode::LDAXD => {
            let de = join_bytes(state.d, state.e);
            state.a = state.mem[de as usize];
        },
        Opcode::DCXD => (state.d, state.e) = split_bytes(join_bytes(state.d, state.e) - 1),
        Opcode::INRE => {
            // check_flag_ac(state.c, state.c + 1, state);
            state.e += 1;
            check_flag_z(state.e, state);
            check_flag_s(state.e, state);
            check_flag_p(state.e, state);
        },
        Opcode::DCRE => {
            // check_flag_ac(state.c, state.c - 1, state);
            state.e -= 1;
            check_flag_z(state.e, state);
            check_flag_s(state.e, state);
            check_flag_p(state.e, state);
        },
        Opcode::MVIE => {
            state.e = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::RAR => {
            check_flag_cy8((state.a as u16).rotate_right(1), state);
            state.a = state.a >> 1;
            state.a |= (state.a << 1) & 0x80;
        },
        Opcode::LXIH => {
            state.h = state.mem[_pc + 2];
            state.l = state.mem[_pc + 1];
            state.pc += 2;
        },
        Opcode::SHLD => {
            let adr = join_bytes(state.mem[_pc + 2], state.mem[_pc + 1]);
            state.mem[adr as usize] = state.l;
            state.mem[(adr + 1) as usize] = state.h;
            state.pc += 2;
        },
        Opcode::INXH => (state.h, state.l) = split_bytes(join_bytes(state.h, state.l) + 1),
        Opcode::INRH => {
            // check_flag_ac(state.h, state.h + 1, state);
            state.h += 1;
            check_flag_z(state.h, state);
            check_flag_s(state.h, state);
            check_flag_p(state.h, state);
        },
        Opcode::DCRH => {
            // check_flag_ac(state.h, state.h - 1, state);
            state.h -= 1;
            check_flag_z(state.h, state);
            check_flag_s(state.h, state);
            check_flag_p(state.h, state);

        },
        Opcode::MVIH => {
            state.h = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::DADH => {
            let hl = join_bytes(state.h, state.l);
            check_flag_cy16( (hl as u32) * 2, state);
            (state.h, state.l) = split_bytes(2*hl);
        },
        Opcode::LHLD => {
            let adr = join_bytes(state.mem[_pc + 2], state.mem[_pc + 1]);
            state.l = state.mem[adr as usize];
            state.h = state.mem[(adr + 1) as usize];
            state.pc += 2;
        },
        Opcode::DCXH => (state.h, state.l) = split_bytes(join_bytes(state.h, state.l) - 1),
        Opcode::INRL => {
            // check_flag_ac(state.l, state.l + 1, state);
            state.l += 1;
            check_flag_z(state.l, state);
            check_flag_s(state.l, state);
            check_flag_p(state.l, state);
        },
        Opcode::DCRL => {
            // check_flag_ac(state.l, state.l - 1, state);
            state.l -= 1;
            check_flag_z(state.l, state);
            check_flag_s(state.l, state);
            check_flag_p(state.l, state);
        },
        Opcode::MVIL => {
            state.l = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::CMA => {
            state.a = !state.a;
        },
        Opcode::LXISP => {
            state.sp = join_bytes(state.mem[_pc + 2], state.mem[_pc + 1]);
            state.pc += 2;
        },
        Opcode::STA => {
            let adr = join_bytes(state.mem[_pc + 2], state.mem[_pc + 1]);
            state.mem[adr as usize] = state.a;
            state.pc += 2;
        },
        Opcode::INXSP => state.sp += 1,
        Opcode::INRM => {
            // check_flag_ac(state.h, state.h + 1, state);
            let adr = join_bytes(state.h, state.l) as usize;
            state.mem[adr] += 1;
            check_flag_z(state.mem[adr], state);
            check_flag_s(state.mem[adr], state);
            check_flag_p(state.mem[adr], state);
        },
        Opcode::DCRM => {
            // check_flag_ac(state.h, state.h - 1, state);
            let adr = join_bytes(state.h, state.l) as usize;
            state.mem[adr] -= 1;
            check_flag_z(state.mem[adr], state);
            check_flag_s(state.mem[adr], state);
            check_flag_p(state.mem[adr], state);
        },
        Opcode::MVIM => {
            let adr = join_bytes(state.h, state.l) as usize;
            state.mem[adr] = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::STC => state.flags.set(FlagType::CY),
        Opcode::DADSP => {
            let hl = join_bytes(state.h, state.l);
            check_flag_cy16( (hl + state.sp) as u32, state);
            (state.h, state.l) = split_bytes(hl + state.sp);
        },
        Opcode::LDA => {
            let adr = join_bytes(state.mem[_pc + 2], state.mem[_pc + 1]);
            state.a = state.mem[adr as usize];
            state.pc += 2;
        },
        Opcode::DCXSP => state.sp -= 1,
        Opcode::INRA => {
            // check_flag_ac(state.a, state.a + 1, state);
            state.a += 1;
            check_flag_z(state.a, state);
            check_flag_s(state.a, state);
            check_flag_p(state.a, state);
        },
        Opcode::DCRA => {
            // check_flag_ac(state.a, state.a - 1, state);
            state.a -= 1;
            check_flag_z(state.a, state);
            check_flag_s(state.a, state);
            check_flag_p(state.a, state);
        },
        Opcode::MVIA => {
            state.a = state.mem[_pc + 1];
            state.pc += 1;
        },
        Opcode::CMC => {
            match state.flags.get(FlagType::CY) == 0 {
                true => state.flags.set(FlagType::CY),
                false => state.flags.unset(FlagType::CY),
            }
        },
        Opcode::MOVBB => state.b = state.b,
        Opcode::MOVBC => state.b = state.c,
        Opcode::MOVBD => state.b = state.d,
        Opcode::MOVBE => state.b = state.e,
        Opcode::MOVBH => state.b = state.h,
        Opcode::MOVBL => state.b = state.l,
        Opcode::MOVBM => state.b = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVBA => state.b = state.a,

        Opcode::MOVCB => state.c = state.b,
        Opcode::MOVCC => state.c = state.c,
        Opcode::MOVCD => state.c = state.d,
        Opcode::MOVCE => state.c = state.e,
        Opcode::MOVCH => state.c = state.h,
        Opcode::MOVCL => state.c = state.l,
        Opcode::MOVCM => state.c = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVCA => state.c = state.a,

        Opcode::MOVDB => state.d = state.b,
        Opcode::MOVDC => state.d = state.c,
        Opcode::MOVDD => state.d = state.d,
        Opcode::MOVDE => state.d = state.e,
        Opcode::MOVDH => state.d = state.h,
        Opcode::MOVDL => state.d = state.l,
        Opcode::MOVDM => state.d = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVDA => state.d = state.a,

        Opcode::MOVEC => state.e = state.c,
        Opcode::MOVEB => state.e = state.b,
        Opcode::MOVED => state.e = state.d,
        Opcode::MOVEE => state.e = state.e,
        Opcode::MOVEH => state.e = state.h,
        Opcode::MOVEL => state.e = state.l,
        Opcode::MOVEM => state.e = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVEA => state.e = state.a,

        Opcode::MOVHB => state.h = state.b,
        Opcode::MOVHC => state.h = state.c,
        Opcode::MOVHD => state.h = state.d,
        Opcode::MOVHE => state.h = state.e,
        Opcode::MOVHH => state.h = state.h,
        Opcode::MOVHL => state.h = state.l,
        Opcode::MOVHM => state.h = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVHA => state.h = state.a,

        Opcode::MOVLB => state.l = state.b,
        Opcode::MOVLC => state.l = state.c,
        Opcode::MOVLD => state.l = state.d,
        Opcode::MOVLE => state.l = state.e,
        Opcode::MOVLH => state.l = state.h,
        Opcode::MOVLL => state.l = state.l,
        Opcode::MOVLM => state.l = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVLA => state.l = state.a,

        Opcode::MOVMB => state.mem[join_bytes(state.h, state.l) as usize] = state.b,
        Opcode::MOVMC => state.mem[join_bytes(state.h, state.l) as usize] = state.c,
        Opcode::MOVMD => state.mem[join_bytes(state.h, state.l) as usize] = state.d,
        Opcode::MOVME => state.mem[join_bytes(state.h, state.l) as usize] = state.e,
        Opcode::MOVMH => state.mem[join_bytes(state.h, state.l) as usize] = state.h,
        Opcode::MOVML => state.mem[join_bytes(state.h, state.l) as usize] = state.l,
        Opcode::HLT => {
            println!("Halted");
            process::exit(-2)
        },
        Opcode::MOVMA => state.mem[join_bytes(state.h, state.l) as usize] = state.a,

        Opcode::MOVAB => state.a = state.b,
        Opcode::MOVAC => state.a = state.c,
        Opcode::MOVAD => state.a = state.d,
        Opcode::MOVAE => state.a = state.e,
        Opcode::MOVAH => state.a = state.h,
        Opcode::MOVAL => state.a = state.l,
        Opcode::MOVAM => state.a = state.mem[join_bytes(state.h, state.l) as usize],
        Opcode::MOVAA => state.a = state.a,

        // Not implemented Instructions
        Opcode::NIMP(x) => {
            println!("Instruction {:#04x} not implemented", x);
            process::exit(-1);  
        },
    };


    state.pc += 1;
}