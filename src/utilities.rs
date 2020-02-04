use std::io::Error;

fn hex_char_to_int(c: char) -> Result<u8, &'static str> {
    let digit: u8 = c.to_ascii_lowercase() as u8;
    if digit >= '0' as u8 && digit <= '9' as u8 {
        return Ok(digit - ('0' as u8))
    } else if digit >= 'a' as u8 && digit <= 'f' as u8 {
        return Ok(digit - ('1' as u8) - ('0' as u8) + 10)
    }
    Err("Invalid character")
} 

//pub fn string_to_bytes(str: String) -> Result<Vec<u8>, Error> {
//    let bytes: Vec<u8> = str.as_bytes().to_vec();
//    Ok(bytes)
//}

pub fn hexstring_to_bytes(str: String) -> Result<Vec<u8>, Error> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut current_byte: u8;
    for (i, c) in str.chars().step_by(2).enumerate() {
        current_byte = hex_char_to_int(c).unwrap() * 16;
        current_byte += hex_char_to_int(str.chars().nth(i * 2 + 1).unwrap()).unwrap();
        bytes.push(current_byte);
    }
    Ok(bytes)
}

pub fn bytes_to_hexstring(bytes: &[u8]) -> String {
    let mut result: String = "".to_string();
    for el in bytes {
        let s = format!("{:02x}", el);
        result.push_str(&s);
    }
    return result;
}
