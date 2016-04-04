use super::{Color, Event, Key, Terminal};
use rustbox::{self, Color as RBColor, RustBox};
use rustbox::{Event as RBEvent, Key as RBKey};

use std::error::Error;

pub struct UnixTerm {
    cursor_x: usize,
    cursor_y: usize,

    bg_color: RBColor,
    fg_color: RBColor,

    terminal: RustBox,
}

impl UnixTerm {
    pub fn new() -> UnixTerm {
        let rustbox = RustBox::init(Default::default())
            .ok()
            .expect("could not open term");
        
        UnixTerm {
            bg_color: convert_color(Color::Default),
            fg_color: convert_color(Color::Default),

            cursor_x: 0,
            cursor_y: 0,

            terminal: rustbox,
        }
    }
}

impl Terminal for UnixTerm {
    fn width(&self)  -> usize { self.terminal.width() }
    fn height(&self) -> usize { self.terminal.height() }

    fn render(&self) {
        self.terminal.present()
    }

    fn clear_fb(&mut self) {} // nop on unix?

    fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_x = col;
        self.cursor_y = row;
    }

    fn color_cursor(&mut self, bg: Color, fg: Color) {
        self.bg_color = convert_color(bg);
        self.fg_color = convert_color(fg);
    }

    fn write_ln(&self, text: &str) {
        self.terminal.print(
            self.cursor_x, 
            self.cursor_y,
            rustbox::RB_BOLD, 
            self.fg_color,
            self.bg_color,
            text
        );
    }

    fn clear_ln(&self) {
        for col in 0..self.width() {
            self.terminal.print(
                col, 
                self.cursor_y, 
                rustbox::RB_BOLD, 
                RBColor::Default, 
                RBColor::Default,
                " "
            );
        }
    }

    fn poll_event(&mut self) -> Result<Event, String> {
        match self.terminal.poll_event(false) {
            Ok(RBEvent::KeyEvent(keycap)) => match keycap {
                RBKey::Char(scalar) => Ok(Event::KeyPress(Key::Char(scalar))),
                RBKey::Enter        => Ok(Event::KeyPress(Key::Enter)),
                RBKey::Backspace    => Ok(Event::KeyPress(Key::Backspace)),

                _ => { Err(format!("unimplemented")) },
            },
            Ok(event) => { Err(format!("unhandled event {:?}", event)) },
            Err(e) => { Err(e.description().to_string()) },
        }
    }
}

fn convert_color(src: Color) -> RBColor {
    match src {
        Color::Default => RBColor::Default,
        Color::White   => RBColor::White,
        Color::Black   => RBColor::Black,
        Color::Red     => RBColor::Red,
        Color::Green   => RBColor::Green,
        Color::Blue    => RBColor::Blue,
        Color::Cyan    => RBColor::Cyan,
        Color::Yellow  => RBColor::Yellow,
        Color::Magenta => RBColor::Magenta,
    }
}
