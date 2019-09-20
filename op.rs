//use tape::State;

pub fn add(state: &State) -> State {
    state.tape[ptr as usize] += 1;
    *state
}

pub fn sub(state: &State) -> State {
    state.tape[ptr as usize] -= 1;
    *state
}
