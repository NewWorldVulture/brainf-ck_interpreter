use std::fs::File;
use std::io::prelude::*;
use std::env;

fn tokenize_char(x:&char) -> &str {
    match x {
        '>' => { //mv::rit();
            "mv::rit()" },
        '<' => { //mv::lft();
            "mv::lft()" },

        '+' => { //op::add();
            "op::add()" },
        '-' => { //op::sub{};
            "op::sub{}" },

        ',' => { //io::get();
            "io::get()" },
        '.' => { //io::prt();
            "io::prt()" },

        '[' => { //lp::str();
            "lp::str()" },
        ']' => { //lp::end();
            "lp::end()" },
            
        _ => "",
    }
}


fn main() -> std::io::Result<()> {
    // Read filename from CLI
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];
    //let input = &args[2];

    // Attempt to open and read the contents of the file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Iterate over characters in program
    for i in contents.chars() {
        println!("{} : {}", i, tokenize_char(&i));
    }
    
    Ok(())
}
