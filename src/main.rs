extern crate anyhow;
extern crate crossterm;

mod platform;
mod repl;

use std::error::Error;
use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::{cursor, event as te, terminal, ExecutableCommand};
use platform::{terror, twrite, tinput, tclear};
use repl::Repl;

fn main() -> anyhow::Result<()> {
    let mut repl  = Repl::new();
    let mut input = format!("");


    stdout()
        .execute(terminal::EnterAlternateScreen)?
        .execute(cursor::DisableBlinking)?
        .execute(cursor::Hide)?
        .flush()?;


    twrite(0, 0, "welcome to ripple!")?;
    twrite(1, 0, "q: quit, d: drop, n: negate")?;

    loop {
        tinput(15, 1, &input[..])?;
        repl.draw();
        // term.render();
        //
        //
        //
        if !te::poll(Duration::from_millis(100))? { continue }

        match te::read()? {
            // input
            te::Event::Key(kc)=> {
                match kc {
                    te::KeyEvent { kind: te::KeyEventKind::Release, .. } => continue,

                    te::KeyEvent { code: te::KeyCode::Char('q'), .. } => break,

                    te::KeyEvent { code: te::KeyCode::Char('d'), .. } => match repl.drop() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? },
                    },

                    te::KeyEvent { code: te::KeyCode::Char('n'), .. } => match repl.negate() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? },
                    },

                    te::KeyEvent { code: te::KeyCode::Char('+'), .. } => match repl.add() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? }
                    },

                    te::KeyEvent { code: te::KeyCode::Char('-'), .. } => match repl.sub() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? }
                    },

                    te::KeyEvent { code: te::KeyCode::Char('*'), .. } =>  match repl.mul() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? }
                    },

                    te::KeyEvent { code: te::KeyCode::Char('/'), .. } =>  match repl.div() {
                        Err(msg) => terror(14, 1, msg)?,
                        _ => { tclear(14, 1)? }
                    },

                    te::KeyEvent { code: te::KeyCode::Char(num), .. } if is_numeric(num) => {
                        tclear(14, 1)?;
                        input.push(num);
                    },

                    te::KeyEvent { code: te::KeyCode::Enter, .. } => {
                        match repl.consume(&input[..]) {
                            Err(e) => terror(14, 1, e.description())?,
                            _ => { tclear(14, 1)? }
                        };

                        tclear(15, 1)?;
                        input.clear();
                    }

                    te::KeyEvent { code: te::KeyCode::Backspace, .. } => {
                        tclear(14, 1)?;
                        tclear(15, 1)?;
                        if input.len() == 0 { continue } // nothing to truncate...
                        let new_len = input.len() - 1; input.truncate(new_len);
                    }

                    _ => terror(14, 1, "unhandled keypress")?,
                }

            }
            _ => {},
        }
    }

    stdout()
        .execute(terminal::LeaveAlternateScreen)?
        .flush()?;

    Ok(()) // TODO: unreachable
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
        '0'..='9' => true,
        '.' => true,
        _ => false,
    }
}
