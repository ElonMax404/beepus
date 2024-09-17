use anyhow::{anyhow, Result};

pub struct Data {
    pub binary: Vec<bool>,
}
impl Data {
    #[allow(unused)]
    pub fn new(bytes: &Vec<u8>) -> Self {
        Data {
            binary: bytes_to_bools(bytes).unwrap(),
        }
    }
}

#[allow(unused)]
fn bytes_to_bools(data: &Vec<u8>) -> Result<Vec<bool>> {
    let mut bits = Vec::new();
    for byte in data {
        for i in 0..8 {
            let bit = (byte >> i) & 1 == 1;

            bits.push(bit);
        }
    }
    Ok(bits)
}

#[allow(unused)]
pub fn bools_to_bytes(bits: &Vec<bool>) -> Result<Vec<u8>> {
    if bits.len() % 8 != 0 {
        return Err(anyhow!("Input length must be a multiple of 8"));
    }

    let mut bytes = Vec::new();

    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for i in 0..8 {
            let bit = chunk[i];

            if bit {
                byte |= 1 << (7 - i); // shift in reverse order
            }
        }

        bytes.push(byte);
    }

    Ok(bytes)
}
