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
pub enum Event {
    KeyPress(Key),
}

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

    fn write_ln(&self, text: &str);
    fn clear_ln(&self);

    fn poll_event(&mut self) -> Result<Event, String>;
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
