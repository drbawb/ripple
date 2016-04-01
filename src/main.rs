#[cfg(unix)] extern crate rustbox;

mod platform;
#[cfg(unix)]    use platform::nix::UnixTerm;
#[cfg(windows)] use platform::win::WinTerm;

use platform::{Color, Terminal};
use platform::{Event, Key};
use std::error::Error;

struct Repl {
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

#[cfg(unix)]    fn init_term() -> UnixTerm { UnixTerm::new() }
#[cfg(windows)] fn init_term() -> WinConsole { WinConsole::new() }

fn main() {
    let mut repl = Repl::new();
    let mut input = format!("");

    // TODO: platform specific
    let mut term = init_term();

    twrite(&mut term, 0, 1, "welcome to ripple!");
    twrite(&mut term, 1, 1, "q: quit, d: drop  ");
    term.render();

    loop {
        repl.draw(&mut term);
        tinput(&mut term, 15, 1, &input[..]);
        term.render();

        // clear buffers
        tclear(&mut term, 14, 1, 0);

        match term.poll_event() {
            // input
            Ok(Event::KeyPress(keycap)) => {
                match keycap {
                    Key::Char('q') => break,
                    Key::Char('d') => repl.drop(),

                    Key::Char('+') => match repl.add() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('-') =>match repl.sub() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('*') => match repl.mul() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char('/') => match repl.div() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {}
                    },

                    Key::Char(num) if is_numeric(num) => input.push(num), 

                    Key::Enter => {
                        match repl.consume(&input[..]) {
                            Err(e) => terror(&mut term, 14, 1, e.description()),
                            _ => {}
                        };

                        tclear(&mut term, 15, 1, input.len());
                        input.clear();
                    }

                    Key::Backspace => {
                        tclear(&mut term, 15, 1, input.len());
                        if input.len() == 0 { continue } // nothing to truncate...
                        let new_len = input.len() - 1; input.truncate(new_len);
                    }

                    _ => terror(&mut term, 14, 1, "unhandled keypress"),
                }
            }

            // catch-all
            //Ok(_)  => terror(&term, 14, 1, "unhandled event"),
            Err(msg) => panic!("err: {}", msg),
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

fn twrite(term: &mut Terminal, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::White);
    term.write_ln(text);
}

fn terror(term: &mut Terminal, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::Red);
    term.write_ln(text);
}

fn tinput(term: &mut Terminal, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::Green);
    term.write_ln(text);
}

fn tclear(term: &mut Terminal, row: usize, col: usize, len: usize) {
    term.move_cursor(row, col);
    term.clear_ln();
}
