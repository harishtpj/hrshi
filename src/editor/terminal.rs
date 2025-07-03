use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::{queue, Command};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType,
    EnterAlternateScreen, LeaveAlternateScreen
};
use std::io::{stdout, Error, Write};

#[derive(Clone, Copy, Default)]
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
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
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

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_cmd(EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_cmd(LeaveAlternateScreen)
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

    pub fn print_row(row: usize, line: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::clear_line()?;
        Self::print(line)
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
