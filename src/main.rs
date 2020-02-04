extern crate serde_json;
extern crate aes_ctr;
mod utilities;
mod data_process;
mod decrypt;
use std::env;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use crate::utilities::bytes_to_hexstring;
use crate::data_process::Data;
use crate::decrypt::derive_key;
use crate::decrypt::check_key;
use crate::decrypt::decrypt;

fn usage_line(name: &String) -> String {
    let msg = format!("Usage: {} <path to JSON file>", name);
    return msg;
}

fn read_json(data: String) -> serde_json::Result<serde_json::value::Value> {
    let v: serde_json::Value = serde_json::from_str(&data)?;
    Ok(serde_json::json!{v})
}

fn read_password() -> Result<String, &'static str> {
    println!("Please enter the password to decrypt the keyfile:");
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read password.");
    let password = line.trim();
    Ok(password.to_string())    
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("{}", usage_line(&args[0]));
        return Err(Error::new(ErrorKind::InvalidInput, "No file specified."));
    }
    let password = read_password().unwrap();
    let filename = &args.get(1);
    let contents = fs::read_to_string(filename.unwrap())?;
    let raw_data = read_json(contents).unwrap();
    let mut data = Data::new(raw_data); 
    data.password = password.into_bytes();
    
    //let key = derive_key(&data).unwrap();
    // For dev purposes, remove
    use crate::utilities::hexstring_to_bytes;
    let key = hexstring_to_bytes("5ae6f8785337645b7cedd53f712863b70cc0615f48f18a3e27a8f922edc13a84".to_string()).unwrap();
    
    if !check_key(&data, &key) {
        eprintln!("Wrong password");
        std::process::exit(1);
    }
    let plaintext: Vec<u8> = decrypt(&data, &key).unwrap();
    println!("{}", bytes_to_hexstring(&plaintext));
    Ok(())
}
