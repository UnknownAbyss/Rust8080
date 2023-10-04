use emulator::arch::state;
mod emulator;


fn main() {
    let mut memory = [0,4,5,5,5];
    let mut state = state::State::new(&mut memory);
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, B: {}", state.flags.reg, state.b);
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, B: {}", state.flags.reg, state.b);
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, B: {}", state.flags.reg, state.b);
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, B: {}", state.flags.reg, state.b);
    emulator::iset::run_op(&mut state);
    println!("Flags: {:#010b}, B: {}", state.flags.reg, state.b);
}
