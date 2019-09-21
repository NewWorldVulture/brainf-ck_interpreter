use std::fs::File;
use std::io::prelude::*;
use std::{ env, mem };

mod tape;
use tape::State;

fn parse_char(instruction:&char, state:&mut State) {
    match instruction {
        '>' => { state.rit(); },
        '<' => { state.lft(); },

        '+' => { state.add(); },
        '-' => { state.sub(); },

        ',' => { state.get(); },
        '.' => { state.prt(); },

        '[' => { state.srt(); },
        ']' => { state.end(); },

        _ => {},    // Skip anything that isn't a recognized instruction
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

    let mut state = tape::State {
        tape: [0].to_vec(),
        ptr: 0u32,
    };
    let mut program_ptr:u32 = 0;

    // Iterate over characters in program
    while (program_ptr as usize) < program.len() {
        let instruction = &program[state.ptr as usize];
        parse_char(&instruction, &mut state);
        //println!("{} : {}", ptr, x);
        program_ptr += 1;
        //println!("{:?} {:?}", state.tape, state.ptr);
    }

    Ok(())
}
