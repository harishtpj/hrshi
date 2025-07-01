use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self{
            should_quit: false,
        }
    }

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
           self.eval_event(&event);
       }
       Ok(())
    }

    fn eval_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, .. }) =  event {
           match code {
               Char('q') if *modifiers == KeyModifiers::CONTROL => {
                   self.should_quit = true;
               }
               _ => ()
           }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position::origin())?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if cur_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}

