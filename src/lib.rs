// Copyright 2020 David Egan
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
// http://www.apache.org/licenses/LICENSE-2.0
//
// Note: In development (i.e. `cargo run`), the scrypt key derivation is very slow.
// It might be useful to skip the key derivation, using a known key derived from a 
// test keyfile. Replace the `let key` assignment in run() with: 
// use crate::utilities::hexstring_to_bytes;
// let key_hexstr = "5ae6f8785337645b7cedd53f712863b70cc0615f48f18a3e27a8f922edc13a84";
// let key = hexstring_to_bytes(key_hexstr.to_string()).unwrap();


extern crate serde_json;
extern crate aes_ctr;
mod utilities;
mod data_process;
mod decrypt;
use std::fs;
use std::io;
use std::io::Error;
use crate::utilities::bytes_to_hexstring;
use crate::data_process::Data;
use crate::decrypt::derive_key;
use crate::decrypt::check_key;
use crate::decrypt::decrypt;

pub struct Config {
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            let msg = format!("Usage: {} <path to JSON file>", args[0]);
            return Err(msg)
        }
        let filepath = args[1].clone();
        Ok(Config {filepath})
    }
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

pub fn run(config: Config) -> Result<(), Error> {
    let password = read_password().unwrap();
    let contents = fs::read_to_string(config.filepath)?;
    let raw_data = read_json(contents).unwrap();
    let mut data = Data::new(raw_data); 
    data.password = password.into_bytes();
    
    let key = derive_key(&data).unwrap();
    if !check_key(&data, &key) {
        eprintln!("Wrong password");
        std::process::exit(1);
    }
    let plaintext: Vec<u8> = decrypt(&data, &key).unwrap();
    println!("{}", bytes_to_hexstring(&plaintext));
    Ok(())
}
