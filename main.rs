use std::fs::File;
use std::io::prelude::*;
use std::{ env, mem };

pub mod tape;
use tape::State;

pub mod mv;
pub mod op;
pub mod io;
pub mod lp;

fn parse_char(instruction:&char, state:&State) -> State {
    match instruction {
        '>' => { mv::rit(&mut state);
            *state },
        '<' => { mv::lft(&mut state);
            *state },

        '+' => { //op::add();
            *state },
        '-' => { //op::sub{};
            *state },

        ',' => { //io::get();   // Accept input
            *state },
        '.' => { io::prt(&mut state);   // Outputs character
            *state },

        '[' => { //lp::str();
            *state },
        ']' => { //lp::end();
            *state },

        _ => *state,    // Skip anything that isn't a recognized instruction
    }
}


fn main() -> std::io::Result<()> {
    // Read filename from CLI
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];
    //let input = &args[2];

    // Attempt to open and read the contents of the file
    let mut file = File::open(filename)?;
    let mut f_contents = String::new();
    file.read_to_string(&mut f_contents)?;
    // Cast `contents` to a vec, then forget the String
    let program: Vec<char> = f_contents.chars().collect();
    mem::forget(f_contents);

    let mut state = State {
        tape: [0].to_vec(),
        ptr: 0u32,
    };

    // Iterate over characters in program
    while (state.ptr as usize) < program.len() {
        let instruction = &program[state.ptr as usize];
        state = parse_char(&instruction, &mut state);
        //println!("{} : {}", ptr, x);
        state.ptr += 1;
    }

    Ok(())
}
