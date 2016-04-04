use platform::Terminal;
use platform::{Color, Event, Key};

use std::ffi::{OsStr, OsString};
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::thread;
use std::time::Duration;


use wio;
use wio::console::{CharInfo, Input, ScreenBuffer};
use winapi::wincon as con;
use winapi::winuser::{VK_BACK, VK_RETURN};

// TODO: assert ascii only
fn w32chars(buf: &str) -> Vec<CharInfo> {
    let fg = 0x0004;
    let bg = 0x0000;
    let attr = (fg | bg) | 0x0008;
    OsStr::new(buf).encode_wide()
        .map(|wchar| { CharInfo::new(wchar, attr) })
        .collect()
}

struct Rect {
    pub top:    i16,
    pub bottom: i16,
    pub left:   i16,
    pub right:  i16,
}

// TODO: there should be a #present() method
// but it looks like the windows console API is immediate mode?
//
// so unless I do internal batching of draw calls
// e.g record [(Coord, (Write | Clear)), ...]
//
// #present() will just be a nop
//
// although that presents a cool opportunity to draw the console
// all in one blit...
//
// e.g I could transform the write-commands directly into a 
// 2D array of CharInfos and pass it to WriteConsoleOutput()
//
// oooooooh...
// NAW FUCK THAT. That's too much work.
//
pub struct WinConsole {
    stdout: ScreenBuffer, // TODO: these should ref the same window but *apparently* they behave differently...
    stdin:  ScreenBuffer, // TODO: these should ref the same window but *apparently* they behave differently...

    cursor_x: usize,
    cursor_y: usize,

    fg_color: Color,
    bg_color: Color,

    input_buf: Vec<Event>,
}

impl WinConsole {
    pub fn new() -> WinConsole {
        WinConsole {
            stdout:    ScreenBuffer::from_stdout().unwrap(),
            stdin:     ScreenBuffer::from_stdin().unwrap(),
            input_buf: vec![],

            cursor_x: 0,
            cursor_y: 0,

            fg_color: Color::Default,
            bg_color: Color::Default,
        }
    }

    // figure out console buffer position
    // HACK: unwrapping the newtype to get at the real winapi bits...
    fn rect_from(info: wio::console::Info) -> Rect {
        unsafe {
            let raw_info: con::CONSOLE_SCREEN_BUFFER_INFO = mem::transmute(info);
            let bufpos = raw_info.srWindow;

            Rect { 
                top:    bufpos.Top,
                bottom: bufpos.Bottom,
                left:   bufpos.Left,
                right:  bufpos.Right
            }
        }
    }
}

impl Terminal for WinConsole {
    fn render(&self) {} // nop on windows
    
    fn clear_fb(&mut self) {
        // TWSS: "[and I'm self so] ... I can just manipulate myself."
        let rect = WinConsole::rect_from(self.stdout.info().unwrap());
        let num_lines = rect.bottom - rect.top;
        assert!(num_lines > 0);

        // clear each row
        // TODO: PERF: could unroll and reuse buf here.
        for row in 0..num_lines {
            self.cursor_y = row as usize;
            self.clear_ln()
        }
    }

    fn width(&self)  -> usize { 0 }
    fn height(&self) -> usize { 0 }

    fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_x = col;
        self.cursor_y = row;
    }

    fn color_cursor(&mut self, bg: Color, fg: Color) { } // todo: fuck...

    fn write_ln(&self, text: &str) { 
        let rect = WinConsole::rect_from(self.stdout.info().unwrap());
        let line = w32chars(text);

        let msg_len  = line.len() as i16;
        let (win_origin_x, win_origin_y) = (
            rect.left + self.cursor_x as i16,
            rect.top + self.cursor_y as i16,
        );

        if msg_len == 0 { return; } // TODO: shit fuck hacky as all get out
        self.stdout.write_output(
            &line[..],
            (msg_len, 1),
            (win_origin_x, win_origin_y)
        ).unwrap();
    }

    fn clear_ln(&self) {
        let rect = WinConsole::rect_from(self.stdout.info().unwrap());

        // build line of spaces that is as wide as the conbuf
        // TODO: format! w/ padding?
        let mut buf = OsString::new();
        for _ in rect.left..rect.right { buf.push(" "); }
       
        // convert to default-styled windows wchars
        let line: Vec<CharInfo> = buf
            .encode_wide()
            .map(|wchar| { CharInfo::new(wchar, 0) })
            .collect();

        // write line
        self.stdout.write_output(
            &line[..],
            (line.len() as i16, 1),
            (rect.left + self.cursor_x as i16, rect.top + self.cursor_y as i16)
        ).unwrap();
    }

    fn poll_event(&mut self) -> Result<Event,String> {
        // block on windows evient loop and drain events to the buffer...
        // TODO: bubble error up
        let mut event_drain: Vec<Event> = vec![];
        let raw_events: Vec<Input>      = self.stdin.read_input().unwrap();
        for event in raw_events.into_iter() {
            // TODO: PERF: could capture win-resize and memoize cursor position
            // instead of computing it on every line...
            // 
            let converted: Option<Event> = match event {
                Input::Key(key_event) if meta_key(key_event) => match key_event.wVirtualKeyCode as i32 {
                    VK_RETURN => Some(Event::KeyPress(Key::Enter)),
                    VK_BACK   => Some(Event::KeyPress(Key::Backspace)),
                    _ => None,
                },

                Input::Key(key_event) if valid_keypress(key_event) => {
                    Some(Event::KeyPress(Key::Char(key_event.UnicodeChar as u8 as char))) // TODO: oh fuck cast
                },
                   

                _ => { None }, // nop, could be used for mouse input, etc.
            };
        
            if let Some(platform_event) = converted { event_drain.push(platform_event); }
        }

        // deal with the buffer ...
        self.input_buf.extend_from_slice(&event_drain[..]);
        match self.input_buf.pop() {
            Some(event) => Ok(event),
            None => Err(format!("no events to return")),
        }
    }

}

fn meta_key(event: con::KEY_EVENT_RECORD) -> bool {
    valid_keypress(event) &&
        (event.wVirtualKeyCode == VK_RETURN as u16 ||
         event.wVirtualKeyCode == VK_BACK as u16)
        
}

fn valid_keypress(event: con::KEY_EVENT_RECORD) -> bool {
    event.bKeyDown == 1 && event.wRepeatCount <= 1 
}
