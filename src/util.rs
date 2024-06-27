#![allow(dead_code)]

use anyhow::{ bail, Result };

pub fn data_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_data(hex: &str) -> Result<Vec<u8>> {
    Ok(hex::decode(hex)?)
}

pub fn data_to_base(buf: &[u8], base: usize) -> Vec<u8> {
    buf.iter()
        .map(|b| (((*b as f64) / 255.0) * ((base - 1) as f64)).round() as u8)
        .collect()
}

pub fn data_to_alphabet(buf: &[u8], base: usize, to_alphabet: fn(usize) -> String) -> String {
    let data = data_to_base(buf, base);
    data.iter()
        .map(|b| to_alphabet((*b).into()))
        .collect()
}

pub fn parse_ints(input: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    for s in input.split_whitespace() {
        let i = s.parse::<usize>()?;
        if !(0..=255).contains(&i) {
            bail!("Integer out of range. Allowed: [0-255]");
        }
        result.push(i as u8);
    }
    Ok(result)
}


pub fn data_to_ints(buf: &[u8], low: usize, high: usize, separator: &str) -> Result<String> {
    if !(low < high && high <= 255) {
        bail!("Int conversion range must be in 0 <= low < high <= 255.");
    }
    let base = high - low + 1;
    let data = data_to_base(buf, base);
    let result = data
        .iter()
        .map(|b| (b + (low as u8)).to_string())
        .collect::<Vec<String>>()
        .join(separator);
    Ok(result)
}

pub fn digits_to_data(in_str: &str, low: usize, high: usize) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    for c in in_str.chars() {
        let n = (c as i32) - ('0' as i32);
        if n < (low as i32) || n > (high as i32) {
            bail!("Invalid digit.");
        }
        result.push(n as u8);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_hex() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(data_to_hex(&data), "00010203040506070809");
    }

    #[test]
    fn test_hex_to_data() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(hex_to_data("00010203040506070809").unwrap(), data);
    }

    #[test]
    fn test_data_to_base() {
        let data = vec![0, 50, 100, 150, 200, 250, 255];
        let expected = vec![0, 1, 2, 3, 4, 5, 5];
        assert_eq!(data_to_base(&data, 6), expected);
    }

    #[test]
    fn test_data_to_alphabet() {
        let data = vec![0, 50, 100, 150, 200, 250, 255];
        let to_alphabet = |n| (((n as u8) + b'a') as char).to_string();
        assert_eq!(data_to_alphabet(&data, 6, to_alphabet), "abcdeff");
    }

    #[test]
    fn test_data_to_ints() {
        let data = vec![0, 50, 100, 150, 200, 250, 255];
        assert_eq!(data_to_ints(&data, 1, 6, ",").unwrap(), "1,2,3,4,5,6,6");
    }

    #[test]
    fn test_digits_to_data() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(digits_to_data("0123456789", 0, 9).unwrap(), data);
    }
}
