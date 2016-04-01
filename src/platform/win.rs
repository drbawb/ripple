use super::{Color, Terminal};
use super::Event;

struct WinConsole;

impl WinConsole {
    pub fn new() -> WinConsole { WinConsole }
}

impl Terminal for WinConsole {
    fn width(&self)  -> usize { 0 }
    fn height(&self) -> usize { 0 }

    fn move_cursor(&mut self, row: usize, col: usize) {}
    fn color_cursor(&mut self, bg: Color, fg: Color) {}
    fn render(&self) {}

    fn poll_event(&self) -> Result<Event, String> { unimplemented!() }
    fn write_ln(&self, text: &str) {}
    fn clear_ln(&self) {}
}
