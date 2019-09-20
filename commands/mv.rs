//use self::State;

pub fn rit(state:&State) -> State {
    // Check if ptr goes above tape.len()
    state.ptr + 1;
    *state
}

pub fn lft(state:&State) -> State {
    // Check if ptr drops below 0. If so, panic
    state.ptr - 1;
    *state
}
