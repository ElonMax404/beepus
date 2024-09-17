use std::io;
use std::io::Write;
#[allow(unused)]
pub fn get_bytes_from_file(filename: Option<String>) -> Vec<u8> {
    let mut path = String::new();
    if let Some(filename) = filename {
        path = filename;
    } else {
        print!("Path to file: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read path.");
    }

    let bytes = match super::encoder::read_bytes(&path.trim()) {
        Err(e) => {
            panic!("{e}")
        }
        Ok(bytes) => bytes,
    };
    bytes
}
