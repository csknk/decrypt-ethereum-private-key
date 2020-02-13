// Copyright 2020 David Egan
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
// http://www.apache.org/licenses/LICENSE-2.0

use serde::{Deserialize, Serialize};
use crate::utilities::hexstring_to_bytes;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub ct: Vec<u8>,
    pub password: Vec<u8>,
    pub salt: Vec<u8>, 
    pub mac: Vec<u8>,
    pub iv: Vec<u8>,
    pub n: u32, 
    pub r: u32, 
    pub p: u32, 
    pub dklen: u32, 
    pub maxmem: u32,
}

impl Default for Data {
    fn default() -> Data {
        Data {
            ct: vec![],
            salt: vec![], 
            mac: vec![],
            iv: vec![],
            password: vec![],
            n: 262144, 
            r: 8,
            p: 1,
            dklen: 32,
            maxmem: 2000000000
        }
    }
}

impl Data {
    pub fn new(raw_data: serde_json::value::Value) -> Data {
        let ct = raw_data["crypto"]["ciphertext"].as_str().unwrap();
        let salt = raw_data["crypto"]["kdfparams"]["salt"].as_str().unwrap();
        let mac = raw_data["crypto"]["mac"].as_str().unwrap();
        let iv = raw_data["crypto"]["cipherparams"]["iv"].as_str().unwrap();
        Data {
            ct: hexstring_to_bytes(ct.to_string()).unwrap(),
            salt: hexstring_to_bytes(salt.to_string()).unwrap(), 
            mac: hexstring_to_bytes(mac.to_string()).unwrap(),
            iv: hexstring_to_bytes(iv.to_string()).unwrap(),
            password: vec![],
            n: raw_data["crypto"]["kdfparams"]["n"].as_u64().unwrap() as u32, 
            r: raw_data["crypto"]["kdfparams"]["r"].as_u64().unwrap() as u32,
            p: raw_data["crypto"]["kdfparams"]["p"].as_u64().unwrap() as u32,
            dklen: raw_data["crypto"]["kdfparams"]["dklen"].as_u64().unwrap() as u32,
            maxmem: 2000000000
        }
    }
}
