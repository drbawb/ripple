use crossterm::{cursor, queue, style, terminal};
use std::io::{stdout, Write};

pub struct Repl {
    stack: Vec<f64>,
}

impl Repl {
    pub fn new() -> Repl {
        Repl { stack: vec![] }
    }

    pub fn consume(&mut self, buf: &str) -> Result<(), anyhow::Error> {
        Ok(self.stack.push(buf.parse()?))
    }

    pub fn draw(&self) -> anyhow::Result<()> {
        let mut stdout = stdout().lock();
        queue!(stdout, style::SetBackgroundColor(style::Color::Black))?;
        queue!(stdout, style::SetForegroundColor(style::Color::White))?;
        queue!(stdout, cursor::SavePosition)?;

        let mut stack = self.stack.iter().rev();
        for i in 0..10 {
            let idx = 9 - i;
            let ofs = 3;

            let prompt = format!("{:02}: ", i+1);
            let prompt_width = prompt.len();
            queue!(stdout, cursor::MoveTo(0, ofs + idx))?;
            queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
            queue!(stdout, style::Print(&prompt[..]))?;

            if let Some(reg) = stack.next() {
                let output   = format!("{}", reg);
                let alt_text = self.alt_repr(*reg);

                queue!(stdout, cursor::MoveTo(prompt_width as u16, ofs + idx))?;
                queue!(stdout, style::Print(&output[..]))?;

                queue!(stdout, cursor::MoveTo(prompt_width as u16 + 30, ofs + idx))?;
                queue!(stdout, style::Print(&alt_text[..]))?;
            }
        }

        queue!(stdout, cursor::RestorePosition)?;
        Ok(stdout.flush()?)
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

    pub fn drop(&mut self) -> Result<(), &'static str> { 
        if self.stack.len() < 1 { return Err("need one operand"); }
        self.stack.pop(); Ok(())
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
