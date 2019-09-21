pub struct State {
    pub tape: Vec<u8>,
    pub ptr: u32,
}

impl State {
    pub fn add(&mut self) {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize].wrapping_add(1u8);
    }

    pub fn sub(&mut self) {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize].wrapping_add(254u8);
    }

    pub fn rit(&mut self) {
        self.ptr += 1;
        // Check if ptr goes above highest cell
        // If so, add another cell onto the tape
        if (self.ptr as usize) >= self.tape.len() {
            self.tape.push(0);
        };
    }

    pub fn lft(&mut self) {
        self.ptr -= 1;
        // Check if ptr drops below 0. If so, panic
        if self.ptr < 0 {
            println!("Pointer head dropped below lowest cell.");
            // TODO: Panic.
            self.ptr +=1;
        };
    }
    
    pub fn get(&mut self) {
        println!("[get_called]");
    }

    pub fn prt(&mut self) {
        println!("{}", self.tape[self.ptr as usize] as char);
    }
    
    pub fn srt(&mut self) {
        println!("[loop_end]");
    }
    
    pub fn end(&mut self) {
        println!("[loop_end]");
    }
}
