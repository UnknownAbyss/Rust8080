use super::arch::state::State;

pub fn join_bytes(reg1: u8, reg2: u8) -> u16 {
    ((reg1 as u16) << 8) | (reg2 as u16)
}

pub fn split_bytes(word: u16) -> (u8, u8) {
    (
        (word >> 8) as u8,
        (word & 0xff) as u8,
    )
}

pub fn check_flag_z(reg: u8, state: &mut State) {
    if reg == 0 {
        state.flags.set(super::arch::flag::FlagType::Z);
    } else {
        state.flags.unset(super::arch::flag::FlagType::Z);
    }
}

pub fn check_flag_s(reg: u8, state: &mut State) { 
    if reg >> 7 == 0b1 {
        state.flags.set(super::arch::flag::FlagType::S);
    } else {
        state.flags.unset(super::arch::flag::FlagType::S);
    }
}

pub fn check_flag_p(reg: u8, state: &mut State) { 
    if reg & 0b1 == 0b0 {
        state.flags.set(super::arch::flag::FlagType::P);
    } else {
        state.flags.unset(super::arch::flag::FlagType::P);
    }
}

pub fn check_flag_cy(reg: u16, state: &mut State) { 
    if reg > 0xff {
        state.flags.set(super::arch::flag::FlagType::CY);
    } else {
        state.flags.unset(super::arch::flag::FlagType::CY);
    }
}

#[allow(dead_code)]
pub fn check_flag_ac(prev: u8, reg: u8, state: &mut State) {
    if (prev & 0x0f) > (reg & 0x0f) {
        state.flags.set(super::arch::flag::FlagType::AC);
    } else {
        state.flags.unset(super::arch::flag::FlagType::AC);
    }
}