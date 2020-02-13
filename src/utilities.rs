// Copyright 2020 David Egan
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
// http://www.apache.org/licenses/LICENSE-2.0

/**
 * Return an integer from a hex character.
 * */
fn hex_char_to_int(c: char) -> Result<u8, &'static str> {
    let digit: u8 = c.to_ascii_lowercase() as u8;
    if digit >= '0' as u8 && digit <= '9' as u8 {
        return Ok(digit - ('0' as u8))
    } else if digit >= 'a' as u8 && digit <= 'f' as u8 {
        return Ok(digit - ('1' as u8) - ('0' as u8) + 10)
    }
    Err("Invalid character in hexstring.")
} 

/**
 * Makes a vector of bytes from a valid hexstring.
 * Walks through characters pairwise - for each pair, the leftmost char represents the
 * factor of 16. The rightmost byte represents units.
 * */
pub fn hexstring_to_bytes(str: String) -> Result<Vec<u8>, &'static str> {
    if str.len() % 2 != 0 {
        return Err("Wrong size hexstring.");
    }
    let mut bytes: Vec<u8> = Vec::new();
    let mut current_byte: u8;
    for (i, c) in str.chars().step_by(2).enumerate() {
        current_byte = hex_char_to_int(c).unwrap() * 16;
        current_byte += hex_char_to_int(str.chars().nth(i * 2 + 1).unwrap()).unwrap();
        bytes.push(current_byte);
    }
    Ok(bytes)
}

/**
 * Return a hexstring representation of a slice of bytes
 * */
pub fn bytes_to_hexstring(bytes: &[u8]) -> String {
    let mut result: String = "".to_string();
    for el in bytes {
        let s = format!("{:02x}", el);
        result.push_str(&s);
    }
    return result;
}
