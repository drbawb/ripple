use std::error::Error;
use platform::Terminal;

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
        let ofs = 3;
        let mut stack = self.stack.iter().rev();
        for i in 0..10 {
            let idx = 9 - i;
            let data = match stack.next() {
                Some(reg) => format!("{}", reg),
                None => "".to_string(),
            };

            let output = format!("{:02}: {}", i+1, data);
            tclear(term, (ofs + idx), 1, 0);
            twrite(term, (ofs + idx), 1, &output[..]);
        }
    }

    // basic ops
    fn grab_two(&mut self) -> (f64, f64) {
        assert!(self.stack.len() >= 2);
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();
        (x, y)
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
