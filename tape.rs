pub struct State {
    pub tape: Vec<u8>,
    pub ptr: u32,
}

impl State {
    pub fn add(&mut self) -> std::io::Result<()> {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize].wrapping_add(1u8);
        Ok(())
    }

    pub fn sub(&mut self) -> std::io::Result<()> {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize].wrapping_neg(1u8);
        Ok(())
    }

    pub fn rit(&mut self) -> std::io::Result<()> {
        self.ptr += 1;
        // Check if ptr goes above highest cell
        // If so, add another cell onto the tape
        if (self.ptr as usize) >= self.tape.len() {
            self.tape.push(0);
        };
        Ok(())
    }

    pub fn lft(&mut self) -> std::io::Result<()> {
        self.ptr -= 1;
        // Check if ptr drops below 0. If so, panic
        if self.ptr < 0 {
            Err("Pointer head dropped below lowest cell. Adjusting.")
        } else {
            Ok(())
        };
    }
    
    pub fn get(&mut self) -> std::io::Result<()> {
        println!("[get() called]");
        Ok(())
    }

    pub fn prt(&mut self) -> std::io::Result<()> {
        print!("{}", self.tape[self.ptr as usize] as char);
        Ok(())
    }
    
    pub fn srt(&mut self) -> std::io::Result<()> {
        if self.tape[self.ptr as usize] == 0 {
            
        }
    }
}

