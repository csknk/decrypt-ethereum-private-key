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
// test keyfile. Replace the `let key` assignment with: 
// use crate::utilities::hexstring_to_bytes;
// let key_hexstr = "5ae6f8785337645b7cedd53f712863b70cc0615f48f18a3e27a8f922edc13a84";
// let key = hexstring_to_bytes(key_hexstr.to_string()).unwrap();

use std::env;
use std::process;
use decrypt_ethereum_private_key::Config;
use decrypt_ethereum_private_key::run;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
 
    Ok(())
}
