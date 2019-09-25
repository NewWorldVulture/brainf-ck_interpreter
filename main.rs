use std::fs::File;
use std::io::prelude::*;
use std::{ env, mem };

mod tape;
use tape::State;

mod prog;
use prog::Program;

pub fn run_instructions(prog:&mut Program, state:&mut State) -> std::result::Result<()> {
    while (prog.ptr as usize) < prog.txt.len() {
        parse_char(&prog, &state);
        prog.next();    // Move program pointer ahead once.
    }
    Ok(())
}

pub fn parse_char(program:&mut Program, state:&mut State) -> std::result::Result<()> {
    let instruction = program.txt[program.ptr as usize];
    match instruction {
        '>' => { state.rit() },
        '<' => { state.lft() },

        '+' => { state.add() },
        '-' => { state.sub() },

        ',' => { state.get() },
        '.' => { state.prt() },

        '[' => { state.srt()
                 run_instructions(); },
        ']' => { state.end()
                 },

        _ => { Ok(()) },    // Skip anything that isn't a recognized instruction
    }
}


fn main() -> std::io::Result<()> {
    // Read filename from CLI
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];
    //let input = &args[2]; // program input

    // Attempt to open and read the contents of the file
    let mut file = File::open(filename)?;
    let mut f_contents = String::new();
    file.read_to_string(&mut f_contents)?;
    // Cast `contents` to a Program, then forget the String
    // let program: Vec<char> = f_contents.chars().collect();
    let program = prog::Program {
        txt: f_contents.chars().collect(),
        ptr: 0;
    };
    mem::forget(f_contents);

    let mut state = tape::State {
        tape: [0].to_vec(),
        ptr: 0u32,
    };
    if program.is_valid() {
        run_instructions(&mut program, &mut state)
}
