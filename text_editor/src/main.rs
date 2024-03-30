use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, Result,
};
use std::io::{self, stdout, Read};
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b00011111
}
fn main() {
    let _ = enable_raw_mode();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:#b} in B \r", b);
        } else {
            println!("{:?} ({}) in C \r", b, c);
        }
        if b == to_ctrl_byte('v') {
            break;
        }
    }
}
