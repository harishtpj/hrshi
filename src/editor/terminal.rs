use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::{queue, Command};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()
    }

    pub fn clear_screen() -> Result<(), Error> {
       Self::queue_cmd(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_cmd(Clear(ClearType::CurrentLine))
    }

    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
        Self::queue_cmd(MoveTo(pos.col as u16, pos.row as u16))
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_cmd(Hide)
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_cmd(Show)
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_cmd(Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;

        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;

        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;

        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    fn queue_cmd(cmd: impl Command) -> Result<(), Error> {
        queue!(stdout(), cmd)
    }
}
