use super::terminal::{Size, Terminal};

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn author_name() -> &'static str {
    env!("CARGO_PKG_AUTHORS").split('<').next().unwrap().trim()
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self { 
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default()
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_centre = height / 3;
        let mut cur_row = 0;
        while cur_row <= height {
            if let Some(ln) = self.buffer.lines.get(cur_row) {
                let mut trunc_ln = ln.clone();
                trunc_ln.truncate(width);
                Self::render_line(cur_row, &trunc_ln);
            } else if cur_row == vertical_centre && self.buffer.is_empty() {
                let msg_lines = Self::build_welcome_msg(width);
                for ln in msg_lines {
                    Self::render_line(cur_row, &ln);

                    cur_row = cur_row.saturating_add(1);
                }
                continue;
            } else {
                Self::render_line(cur_row, "~");
            }

            cur_row = cur_row.saturating_add(1);
        }
        self.needs_redraw = false;
    }

    pub fn load(&mut self, fname: &str) {
        if let Ok(buf) = Buffer::load(fname)  {
            self.buffer = buf;
        }
        self.needs_redraw = true;
    }

    fn push_if_fits(vec: &mut Vec<String>, msg: &str, width: usize) {
        let cols = width.saturating_sub(1);
        let len = msg.len();
        if width <= len {
            vec.push("~".to_string());
        } else {
            let msg = format!("~{msg:^cols$}");
            vec.push(msg);
        }
    }

    fn build_welcome_msg(width: usize) -> Vec<String> {
        if width == 0 {
            return vec![" ".to_string()];
        }

        let mut ret_list: Vec<String> = Vec::new();

        let mut msg = format!("Welcome to {NAME} editor v{VERSION}");
        Self::push_if_fits(&mut ret_list, &msg, width);

        msg = format!("Written by {}", author_name());
        Self::push_if_fits(&mut ret_list, &msg, width);

        ret_list.push("~".to_string());
        
        // TODO: Replace it with some quotes
        Self::push_if_fits(&mut ret_list, "Start writing something now!", width);

        ret_list
    }
}
