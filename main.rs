// Created by Ada.

use std::fs::File;
use std::io::prelude::*;
use std::{ env, mem };

mod tape;
use tape::State;

mod prog;
use prog::Program;

// Main program runner. Treats loops and the main program the same way
pub fn run_program(program:&mut Program, state:&mut State, is_main:bool) -> Result< (), &'static str > {
    while (program.ptr as usize) < program.txt.len() {
        let result = parse_char(program, state, is_main);

        let result = match result {
            Ok(()) => { },
            Err("Loop") => program.ptr = 0,
            _ => { },
        };

        program.next();    // Move program pointer ahead once.
        if state.tape[state.ptr as usize] == 0 && is_main == false {
            break
        }
    }

    Ok(())
}

pub fn call_loop(program:&mut Program, state:&mut State, is_main:bool) -> Result< (), &'static str > {
    // Find length and contents of the loop
    let mut loop_length: usize = 0;
    let mut depth = 1;
    for chr in program.txt[program.ptr as usize ..].to_vec() {
        loop_length +=1;
        match chr {
            '[' => { depth += 1; },
            ']' => { depth -= 1;
                     if depth == 0 { break }
                   },
             _ => { },
        };
    }

    // Set up new program for inner loop
    let mut loop_txt = prog::Program {
        txt: program.txt[program.ptr as usize +1 .. program.ptr as usize +loop_length].to_vec(),
        ptr: 0,
        inpt: program.inpt.clone().to_string()
    };

    // Move program pointer up to the end of the loop to avoid forgetting to later
    program.ptr = program.ptr + loop_length as u32;

    run_program(&mut loop_txt, state, false);
    return Ok(())
}

pub fn parse_char(program:&mut Program, state:&mut State, is_main:bool) -> Result< (), &'static str > {
    let instruction = &program.txt[program.ptr as usize];
    match instruction {
        '>' => { state.rit() },
        '<' => { state.lft() },

        '+' => { state.add() },
        '-' => { state.sub() },

        ',' => { state.get() },
        '.' => { state.prt() },

        '[' => { call_loop(program, state, is_main) },
        ']' => { state.end(is_main) },

        _ => { Ok(()) },    // Skip anything that isn't a recognized instruction
    }
}


fn main() -> std::io::Result<()> {
    // Read filename from CLI
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];
    let input: &str = &args[2]; // program input

    // Attempt to open and read the contents of the file
    let mut file = File::open(filename)?;
    let mut f_contents = String::new();
    file.read_to_string(&mut f_contents)?;
    // Cast `contents` to a Program, then forget the String
    // let program: Vec<char> = f_contents.chars().collect();
    let mut program = prog::Program {
        txt: f_contents.chars().collect(),
        ptr: 0,
        inpt: input.to_string(),
    };
    mem::forget(f_contents);

    let mut state = tape::State {
        tape: [0].to_vec(),
        ptr: 0i64,
    };

    if program.is_valid() {
        run_program(&mut program, &mut state, true);
        // Not yet handling errors correctly
        Ok(())
    } else {
        println!("Compile-time Error. Aborting Program.");
        Ok(())
    }
}
