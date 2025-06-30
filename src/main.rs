use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
           Ok(b) => {
               let c = b as char;
               if c.is_control() {
                   println!("Binary: {b:08b} ASCII: {b:#03} \r");
               } else {
                   println!("Binary: {b:08b} ASCII: {b:#03} Character: {c:#?}\r");
               }
               if c == 'q' {
                   break;
               }
           } 
           Err(e) => println!("Error: {e}")
        }
    }
    disable_raw_mode().unwrap();
}
