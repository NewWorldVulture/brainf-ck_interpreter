//use self::State;

pub fn get() {

}

pub fn prt(state:&State) -> State {
    let printable = state.tape[state.ptr as usize];
    print!("{}", printable as char);
    *state
}
