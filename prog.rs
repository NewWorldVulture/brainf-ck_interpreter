pub struct Program {
    pub txt: Vec<char>,
    pub ptr: u32,
}

impl Program {
    pub fn is_valid(&self) -> std::io::Result<()> {
        let mut depth:i8 = 0;
        let mut print_out:bool = false;
        
        for chr in self.txt {
            match chr {
                '[' => { depth += 1; },
                ']' => { depth -= 1;
                         if depth < 0 {
                             Err("Error: Extra `]` in program. Aborting.")
                         }
                       },
                '.' => { print_out = true; },
                _ => { },
            }
        };
        if depth > 0 {
            Err("Error: Unterminated Loop in program. Needs matching `]`. Aborting.")
        //} if else print_out == False {
        //    Err("Warning: Program never prints out. Continuing.")
        } else {
            Ok(())
        }
    }

    pub fn next(&self) {
        self.ptr += 1;
        Ok(())
    }
}
