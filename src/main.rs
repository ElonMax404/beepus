mod data;
pub mod encoder;
mod filetosound;
mod parse;

use std::io;
use std::io::Write;

fn main() {
    println!("Hello. What are we doing today?");
    println!("1) File to beeps.");
    println!("2) Beeps to file.");
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        let mut mode = String::new();
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read response.");
        match mode.as_str().trim() {
            "1" => {
                filetosound::file_to_sound();
                break;
            }
            "2" => break,
            _ => {
                println!("Are you retarded?");
                continue;
            }
        }
    }
}
