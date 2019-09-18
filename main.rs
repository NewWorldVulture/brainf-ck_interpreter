use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::num::Wrapping;

fn tokenize_char(x:char) -> &'str {
    match x {
        '>' => "mv rit",
        '<' => "mv let",

        '+' => "op add",
        '-' => "op sub",

        ',' => "io get",
        '.' => "io put",

        '[' => "lp str",
        ']' => "lp end",
        _ => (),
    }
}


fn main() -> std::io::Result<()> {
    // Read filename from CLI
    let args:Vec<String> = env::args().collect();
    let FILENAME = &args[1];

    // Attempt to open and read the contents of the file
    let mut file = File::open(FILENAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Iterate over characters in program
    for i in contents.chars() {
        println!("{}", i)

    }
    Ok(())
}
