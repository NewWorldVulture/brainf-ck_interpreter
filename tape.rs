pub struct State {
    pub tape: Vec<u8>,
    pub ptr: u32,
}

impl State {
    pub fn add(&mut self) {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize] += 1;
    }

    pub fn sub(&mut self) {
        let ptr_val:u32 = self.ptr;
        self.tape[ptr_val as usize] -= 1;
    }

    pub fn rit(&mut self) {
        // Check if ptr goes above tape.len()
        self.ptr + 1;
    }

    pub fn lft(&mut self) {
        // Check if ptr drops below 0. If so, panic
        self.ptr - 1;
    }

    pub fn prt(&self) {
        print!("{}", self.tape[self.ptr as usize] as char);
    }

}
