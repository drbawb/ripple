use crossterm::{
    cursor,
    queue,
    style, terminal,
};

use std::io::{stdout, Write};


pub fn twrite(row: u16, col: u16, text: &str) -> anyhow::Result<()> {
    let mut stdout = stdout().lock();

    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(col, row))?;
    queue!(stdout, style::SetForegroundColor(style::Color::White))?;
    queue!(stdout, style::SetBackgroundColor(style::Color::Black))?;
    queue!(stdout, style::Print(text))?;
    queue!(stdout, cursor::RestorePosition)?;

    Ok(stdout.flush()?)
}

pub fn terror(row: u16, col: u16, text: &str) -> anyhow::Result<()> {
    let mut stdout = stdout().lock();

    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(col, row))?;
    queue!(stdout, style::SetForegroundColor(style::Color::Red))?;
    queue!(stdout, style::SetBackgroundColor(style::Color::Black))?;
    queue!(stdout, style::Print(text))?;
    queue!(stdout, cursor::RestorePosition)?;

    Ok(stdout.flush()?)
}

pub fn tinput(row: u16, col: u16, text: &str) -> anyhow::Result<()> {
    let mut stdout = stdout().lock();

    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(col, row))?;
    queue!(stdout, style::SetForegroundColor(style::Color::Green))?;
    queue!(stdout, style::SetBackgroundColor(style::Color::Black))?;
    queue!(stdout, style::Print(text))?;
    queue!(stdout, cursor::RestorePosition)?;
    
    Ok(stdout.flush()?)
}

pub fn tclear(row: u16, col: u16) -> anyhow::Result<()> {
    let mut stdout = stdout().lock();

    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(col, row))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
    queue!(stdout, cursor::RestorePosition)?;
    
    Ok(stdout.flush()?)
}
