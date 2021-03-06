#[cfg(unix)] extern crate rustbox;
#[cfg(windows)] extern crate wio;
#[cfg(windows)] extern crate winapi;

mod platform;
mod repl;

use std::error::Error;

use platform::{Terminal, Event, Key};
use platform::{terror, twrite, tinput, tclear};
use repl::Repl;

#[cfg(unix)] use platform::nix::UnixTerm;
#[cfg(unix)] fn init_term() -> UnixTerm { UnixTerm::new() }

#[cfg(windows)] use platform::win::WinConsole;
#[cfg(windows)] fn init_term() -> WinConsole { WinConsole::new() }

fn main() {
    let mut repl  = Repl::new();
    let mut input = format!("");
    let mut term  = init_term();

    term.clear_fb();
    twrite(&mut term, 0, 1, "welcome to ripple!");
    twrite(&mut term, 1, 1, "q: quit, d: drop, n: negate");
    term.render();

    loop {
        repl.draw(&mut term);
        tinput(&mut term, 15, 1, &input[..]);
        term.render();

        match term.poll_event() {
            // input
            Ok(Event::KeyPress(keycap)) => {
                // clear the error buffer once we have more input
                tclear(&mut term, 14, 1);

                match keycap {
                    Key::Char('q') => break,

                    Key::Char('d') => match repl.drop() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {},
                    },

                    Key::Char('n') => match repl.negate() {
                        Err(msg) => terror(&mut term, 14, 1, msg),
                        _ => {},
                    },

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

                        tclear(&mut term, 15, 1);
                        input.clear();
                    }

                    Key::Backspace => {
                        tclear(&mut term, 15, 1);
                        if input.len() == 0 { continue } // nothing to truncate...
                        let new_len = input.len() - 1; input.truncate(new_len);
                    }

                    _ => terror(&mut term, 14, 1, "unhandled keypress"),
                }
            }

            // catch-all
            //Ok(_)  => terror(&term, 14, 1, "unhandled event"),
            Err(_message) => {},
        }
    }
}

// TODO: better input handling, more input modes
// e.g: infix, lisp-y prefix or postfix, scientific notation, etc.
// e.g: only permit one decimal point
// e.g: make sure input register is always a valid number b/c some day
//      input register might just be another part of the stack 
//      (like old HP calculators.)
//
fn is_numeric(input: char) -> bool {
    match input {
        '0'...'9' => true,
        '.' => true,
        _ => false,
    }
}
