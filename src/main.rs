extern crate rustbox;

use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::{Event, Key};

struct Repl {
    stack: Vec<f64>, // todo: deque?
}

impl Repl {
    pub fn new() -> Repl {
        Repl { stack: vec![] }
    }

    pub fn consume(&mut self, buf: &str) -> Result<(), Box<Error>> {
        Ok(self.stack.push(try!(buf.parse())))
    }

    pub fn draw(&self, term: &RustBox) {
        let ofs = 3;
        let mut stack = self.stack.iter().rev();
        for i in 0..10 {
            let idx = 9 - i;
            let data = match stack.next() {
                Some(reg) => format!("{}", reg),
                None => "".to_string(),
            };

            let output = format!("{:02}: {}", i+1, data);
            tclear(term, (ofs + idx), 1, term.width());
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



fn main() {
    let mut repl = Repl::new();
    let mut input = format!("");

    let term = RustBox::init(Default::default())
                   .ok()
                   .expect("could not open term...");

    twrite(&term, 0, 1, "ripple - input mode");
    twrite(&term, 1, 1, "q: quit, ?: help   ");
    term.present();

    loop {
        repl.draw(&term);
        tinput(&term, 15, 1, &input[..]);
        term.present();

        // clear buffers
        tclear(&term, 14, 1, term.width());

        match term.poll_event(false) {
            Ok(Event::KeyEvent(keycap)) => {
                match keycap {
                    Key::Char('q') => break,

                    Key::Char('+') => match repl.add() {
                        Err(msg) => terror(&term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('-') =>match repl.sub() {
                        Err(msg) => terror(&term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('*') => match repl.mul() {
                        Err(msg) => terror(&term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('/') => match repl.div() {
                        Err(msg) => terror(&term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char(num) if is_numeric(num) => input.push(num), 

                    Key::Enter => {
                        match repl.consume(&input[..]) {
                            Err(e) => terror(&term, 14, 1, e.description()),
                            _ => {}
                        };

                        tclear(&term, 15, 1, input.len());
                        input.clear();
                    }

                    Key::Backspace => {
                        tclear(&term, 15, 1, input.len());
                        let new_length = input.len() - 1;
                        if new_length >= 0 { input.truncate(new_length); }
                    }

                    _ => terror(&term, 14, 1, "unhandled keypress"),
                }
            }

            Ok(_) => {
                terror(&term, 14, 1, "unhandled event");
            }

            Err(e) => {
                panic!("err: {}", e.description());
            }
        }
    }
}

fn is_numeric(input: char) -> bool {
    match input {
        '0'...'9' => true,
        '.' => true, // TODO: not technically true, can only have one ?
        _ => false,
    }
}

fn twrite(term: &RustBox, row: usize, col: usize, text: &str) {
    term.print(col, row, rustbox::RB_BOLD, Color::White, Color::Black, text);
}

fn terror(term: &RustBox, row: usize, col: usize, text: &str) {
    term.print(col, row, rustbox::RB_BOLD, Color::Red, Color::Black, text);
}

fn tinput(term: &RustBox, row: usize, col: usize, text: &str) {
    term.print(col, row, rustbox::RB_BOLD, Color::White, Color::Black, text);
}

fn tclear(term: &RustBox, row: usize, col: usize, len: usize) {
    for i in 0..len {
        term.print(col + i,
                   row,
                   rustbox::RB_BOLD,
                   Color::Default,
                   Color::Default,
                   " ");
    }
}
