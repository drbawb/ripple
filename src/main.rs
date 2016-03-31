extern crate rustbox;

use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::{Event, Key};

struct Repl {
    stack: Vec<f64>, // todo: deque?
}

macro_rules! impl_bin_op {
    ($x:ident, $y:ident, $result:expr) => {{
         match ($x,$y) {
            (Some($x), Some($y)) => { Ok($result) },
            _ => { Err("need two operands on stack") },
        }
    }}
}

impl Repl {
    pub fn new() -> Repl {
        Repl { stack: vec![] }
    }

    // basic ops
    pub fn add(&mut self) -> Result<(), &'static str> {
        let y = self.stack.pop();
        let x = self.stack.pop();
        impl_bin_op!(x, y, self.stack.push(x + y))
    }

    pub fn sub(&mut self) -> Result<(), &'static str> {
        let y = self.stack.pop();
        let x = self.stack.pop();
        impl_bin_op!(x, y, self.stack.push(x - y))
    }

    pub fn mul(&mut self) -> Result<(), &'static str> {
        let y = self.stack.pop();
        let x = self.stack.pop();
        impl_bin_op!(x, y, self.stack.push(x * y))
    }

    pub fn div(&mut self) -> Result<(), &'static str> {
        let y = self.stack.pop();
        let x = self.stack.pop();
        impl_bin_op!(x, y, self.stack.push(x / y))
    }
}



fn main() {
    let term = RustBox::init(Default::default())
        .ok().expect("could not open term...");


    twrite(&term, 1, 1, "hello, world");
    twrite(&term, 3, 1, "press 'q' to quit");
    twrite(&term, 4, 1, "press 'l' to lol");
    term.present();

    let mut input = format!("");

    loop {
        match term.poll_event(false) {
            Ok(Event::KeyEvent(keycap)) => {
                match keycap {
                    Key::Char('q') => { break; },
                    Key::Char('l') => {
                        twrite(&term, 5, 1, "what the shit?");
                    },

                    Key::Char(num) if is_numeric(num) => {
                        input.push(num);
                        tinput(&term, 10, 1, &input[..]);
                    },

                    Key::Enter => {
                        tclear(&term, 10, 1, input.len());
                        input.clear();
                    },

                    _ => {},
                }
            },

            Ok(_)  => { 
                println!("unknown key"); 
            },

            Err(e) => { panic!("err: {}", e.description()); },
        }

        term.present();
    }
}

fn is_numeric(input: char) -> bool {
    match input {
         '0'...'9' => { true },
         _ => { false },
    }
}

fn twrite(term: &RustBox, row: usize, col: usize, text: &str) {
    term.print(col, row, rustbox::RB_BOLD, Color::White, Color::Black, text);
}

fn tinput(term: &RustBox, row: usize, col: usize, text: &str) {
    term.print(col, row, rustbox::RB_BOLD, Color::White, Color::Black, text);
}

fn tclear(term: &RustBox, row: usize, col: usize, len: usize) {
    for i in 0..len {
        term.print(col+i, row, rustbox::RB_BOLD, Color::Default, Color::Default, " ");
    }
}
