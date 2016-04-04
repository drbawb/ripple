#[cfg(unix)]    pub mod nix;
#[cfg(windows)] pub mod win;

#[allow(dead_code)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

// TODO: hold, release, repeat (winapi?)
#[derive(Clone,Copy)]
pub enum Event {
    KeyPress(Key),
}

#[derive(Clone,Copy)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
}

pub trait Terminal {
    fn width(&self)  -> usize;
    fn height(&self) -> usize;

    fn move_cursor(&mut self, row: usize, col: usize);
    fn color_cursor(&mut self, bg: Color, fg: Color);
    fn render(&self);
    fn clear_fb(&mut self);

    fn write_ln(&self, text: &str);
    fn clear_ln(&self);

    fn poll_event(&mut self) -> Result<Event, String>;
}

pub fn twrite<T: Terminal>(term: &mut T, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::White);
    term.write_ln(text);
}

pub fn terror<T: Terminal>(term: &mut T, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::Red);
    term.write_ln(text);
}

pub fn tinput<T: Terminal>(term: &mut T, row: usize, col: usize, text: &str) {
    term.move_cursor(row, col);
    term.color_cursor(Color::Black, Color::Green);
    term.write_ln(text);
}

pub fn tclear<T: Terminal>(term: &mut T, row: usize, col: usize) {
    term.move_cursor(row, col);
    term.clear_ln();
}
