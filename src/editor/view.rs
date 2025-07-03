use super::terminal::{Size, Terminal};
use std::io::Error;

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn author_name() -> &'static str {
    env!("CARGO_PKG_AUTHORS").split('<').next().unwrap().trim()
}

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render_welcome_screen() -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        let mut cur_row = 0;
        while cur_row <= height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if cur_row == height / 3 {
                let offset = Self::draw_welcome_msg()?;
                cur_row += offset;
                continue;
            } else {
                Self::draw_empty_row()?;
            }

            if cur_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
            cur_row += 1;
        }
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(cur_row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }

            if cur_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()
        } else {
            self.render_buffer()
        }
    }

    pub fn load(&mut self, fname: &str) {
        if let Ok(buf) = Buffer::load(fname)  {
            self.buffer = buf;
        }
    }

    fn draw_welcome_msg() -> Result<usize, Error> {
        let mut msg = format!("Welcome to {NAME} editor v{VERSION}");
        let cols = Terminal::size()?.width.saturating_sub(1);

        msg = format!("~{msg:^cols$}\r\n");
        Terminal::print(&msg)?; // Line 1

        msg = format!("Written by {}", author_name());
        msg = format!("~{msg:^cols$}\r\n");
        Terminal::print(&msg)?; // Line 2

        Terminal::print("~\r\n")?; // Line 3
        
        // TODO: Replace it with some quotes
        msg = format!("~{:^cols$}\r\n", "Start writing something now!");
        Terminal::print(&msg)?; // Line 4

        return Ok(4)
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }
}
