use std::error::Error;
use platform::{self, Terminal};

pub struct Repl {
    stack: Vec<f64>,
}

impl Repl {
    pub fn new() -> Repl {
        Repl { stack: vec![] }
    }

    pub fn consume(&mut self, buf: &str) -> Result<(), Box<Error>> {
        Ok(self.stack.push(try!(buf.parse())))
    }

    pub fn drop(&mut self) { self.stack.pop(); }

    pub fn draw(&self, term: &mut Terminal) {
        let mut stack = self.stack.iter().rev();
        for i in 0..10 {
            let idx = 9 - i;
            let ofs = 3;

            let prompt = format!("{:02}: ", i+1);
            let prompt_width = prompt.len();
            term.move_cursor((ofs + idx), 0);
            term.clear_ln();
            term.write_ln(&prompt[..]);

            if let Some(reg) = stack.next() {
                let output   = format!("{}", reg);
                let alt_text = self.alt_repr(*reg);

                term.move_cursor((ofs + idx), prompt_width);
                term.write_ln(&output[..]);

                term.move_cursor((ofs + idx), prompt_width + 30);
                term.write_ln(&alt_text[..]);
            }
        }
    }

    // offset x = 20, draw numbers in different modes, e.g binary, hex, etc.
    fn alt_repr(&self, value: f64) -> String {
        let reg = value.floor() as i64;
        format!("0x{:016x} | 0b{:08b}", reg, reg)
    }

    // basic ops
    fn grab_two(&mut self) -> (f64, f64) {
        assert!(self.stack.len() >= 2);
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();
        (x, y)
    }

    pub fn negate(&mut self) -> Result<(), &'static str> {
        if self.stack.len() < 1 { return Err("need one operand"); }
        let x = self.stack.pop().unwrap();
        Ok(self.stack.push(-x))
    }

    pub fn add(&mut self) -> Result<(), &'static str> {
        if self.stack.len() < 2 { return Err("need two operands"); }
        let (x, y) = self.grab_two();
        Ok(self.stack.push(x + y))
    }

    pub fn sub(&mut self) -> Result<(), &'static str> {
        if self.stack.len() < 2 { return Err("need two operands"); }
        let (x, y) = self.grab_two();
        Ok(self.stack.push(x - y))
    }

    pub fn mul(&mut self) -> Result<(), &'static str> {
        if self.stack.len() < 2 { return Err("need two operands"); }
        let (x, y) = self.grab_two();
        Ok(self.stack.push(x * y))
    }

    pub fn div(&mut self) -> Result<(), &'static str> {
        if self.stack.len() < 2 { return Err("need two operands"); }
        let (x, y) = self.grab_two();
        Ok(self.stack.push(x / y))
    }
}
