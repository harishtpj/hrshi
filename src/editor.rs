use core::cmp::min;
use std::{
    env, 
    io::Error,
    panic::{set_hook, take_hook}
};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

mod terminal;
mod view;
use terminal::{Terminal, Size, Position};
use view::View;


#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let cur_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            cur_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut view = View::default();

        let args: Vec<String> = env::args().collect();
        if let Some(fname) = args.get(1) {
            view.load(fname);
        }

        Ok(Self { 
            should_quit: false, 
            location: Location::default(), 
            view
        })
    }

    pub fn run(&mut self) {
       loop {
           self.refresh_screen();
           if self.should_quit {
               break;
           }
           match read() {
               Ok(event) => self.eval_event(event),
               Err(e) => {
                   #[cfg(debug_assertions)]
                   panic!("Could not read event: {e:?}");
               }
           }
       }
    }

    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
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
    }

    #[allow(clippy::needless_pass_by_value)]
    fn eval_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                       | KeyCode::Down
                       | KeyCode::Left
                       | KeyCode::Right
                       | KeyCode::PageUp
                       | KeyCode::PageDown
                       | KeyCode::End
                       | KeyCode::Home,
                       _
                ) => { self.move_point(code); }
                _ => {}
            }
            Event::Resize(w_u16, h_u16) => {
                #[allow(clippy::as_conversions)]
                let height = h_u16 as usize;

                #[allow(clippy::as_conversions)]
                let width = w_u16 as usize;

                self.view.resize(Size { height, width });
            }
            _ => {}
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(Position { 
            col: self.location.x, 
            row: self.location.y
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}

