pub struct Program {
    pub txt: Vec<char>,
    pub ptr: u32,
    pub inpt: String,
}

impl Program {
    pub fn is_valid(&self) -> bool {
        let mut depth:i8 = 0;

        // Step through entire program once to confirm that all loops terminate.
        for chr in &self.txt {
            match chr {
                '[' => { depth += 1; },
                ']' => { depth -= 1;
                         if depth < 0 {
                             return false
                         }
                       },
                _ => { },
            }
        };
        if depth > 0 {
            false
        } else {
            true
        }
    }

    pub fn next(&mut self) {
        self.ptr += 1;
    }
}
