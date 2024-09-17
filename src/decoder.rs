use std::fs;
use anyhow::anyhow;

pub fn write_bytes(path: &str, bytes: Vec<u8>){
    fs::write(path, bytes).expect("Failed to write bytes.");
}