pub struct State {
    pub tape: Vec<u8>,
    pub ptr: i64,
}

impl State {
    // Add one to the current cell. Should not error.
    pub fn add(&mut self) -> Result< (), &'static str > {
        let ptr_val = self.ptr;
        self.tape[ptr_val as usize].wrapping_add(1u8);
        Ok(())
    }

    // Subtract one from the current cell. Should not error.
    pub fn sub(&mut self) -> Result< (), &'static str > {
        let ptr_val = self.ptr;
        self.tape[ptr_val as usize].wrapping_add(255u8); // adding 255 == adding -1
        Ok(())
    }

    // Move the pointer to the right
    pub fn rit(&mut self) -> Result< (), &'static str > {
        self.ptr += 1;
        // Check if ptr goes above highest cell
        // If so, add another cell onto the tape
        if (self.ptr as usize) >= self.tape.len() {
            self.tape.push(0);
        };
        Ok(())
    }

    // Move the pointer to the left
    pub fn lft(&mut self) -> Result< (), &'static str > {
        // Check if ptr drops below the furthest-left cell. If so, panic.
        self.ptr -= 1;
        if self.ptr < 0 {
            println!("Runtime Error: `<` Pointer moved off of tape.");
            return Err("Runtime Error: `<` Pointer moved off of tape.")
        };
        Ok(())
    }

    // Get one
    // TODO! : Finish this
    pub fn get(&mut self) -> Result< (), &'static str > {
        println!("[get() called]");
        Ok(())
    }

    // Print the current cell's ASCII value as a character
    pub fn prt(&mut self) -> Result< (), &'static str > {
        print!("{}", self.tape[self.ptr as usize] as char);
        Ok(())
    }

    // Starting a loop is defined in

    // End a loop
    pub fn end(&mut self, is_main:bool) -> Result< (), &'static str > {
        if self.tape[self.ptr as usize] == 0 {
            println!("Ending loop here.");
            if (is_main == false) && (self.tape[self.ptr as usize] == 0) {
                return Err("Loop");
            }
        }
        Ok(())
    }
}
