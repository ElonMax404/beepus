use anyhow::anyhow;
use std::fs;

pub fn read_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let byte_data = match fs::read(path) {
        Ok(data) => data,
        Err(e) => {
            panic!("{e}");
        }
    };
    if byte_data.is_empty() {
        return Err(anyhow!("File is empty!"));
    }
    println!("Bytes read successfully.");
    println!("Byte length: {}", byte_data.len());
    Ok(byte_data)
}
