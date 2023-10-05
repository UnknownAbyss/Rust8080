use emulator::arch::state;
mod emulator;


fn main() {
    let mut memory = [0,0x1f,0x1f,0x1f];
    let mut state = state::State::new(&mut memory);
    
    state.a = 0b11000010;
    
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, A: {:#010b}", state.flags.reg, state.a);

    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, A: {:#010b}", state.flags.reg, state.a);
    
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, A: {:#010b}", state.flags.reg, state.a);

    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, A: {:#010b}", state.flags.reg, state.a);
}
