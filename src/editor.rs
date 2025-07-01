use core::cmp::min;
use std::io::Error;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};

mod terminal;
use terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let res = self.repl();
        Terminal::terminate().unwrap();
        res.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
       loop {
           self.refresh_screen()?;
           if self.should_quit {
               break;
           }
           let event = read()?;
           self.eval_event(&event)?;
       }
       Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn eval_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) =  event {
           match code {
               KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                   self.should_quit = true;
               }
               KeyCode::Up
                   | KeyCode::Down
                   | KeyCode::Left
                   | KeyCode::Right
                   | KeyCode::PageUp
                   | KeyCode::PageDown
                   | KeyCode::End
                   | KeyCode::Home => {
                       self.move_point(*code)?;
               }
               _ => ()
           }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position { 
                col: self.location.x, 
                row: self.location.y
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let mut msg = format!("{NAME} editor -- version {VERSION}");
        let cols = Terminal::size()?.width.saturating_sub(1);
        msg = format!("~{msg:^cols$}");
        Terminal::print(msg)
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if cur_row == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if cur_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}

