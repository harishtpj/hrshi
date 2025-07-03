use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(fname: &str) -> Result<Self, Error> {
        let contents = read_to_string(fname)?;
        let mut lines = Vec::new();
        for ln in contents.lines() {
            lines.push(String::from(ln));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

