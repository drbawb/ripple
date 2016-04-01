pub mod nix;

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

    fn poll_event(&self) -> Result<Event, String>;
}
