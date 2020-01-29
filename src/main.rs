extern crate serde_json;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
//use serde_json::{Result, Value};
//use serde_json;


fn usage_line(name: &String) -> String {
    format!("Usage: {} <path to JSON file>", name)
}

fn usage_error(name: &String) -> Error {
    let msg = format!("Usage: {} <path to JSON file>", name);
    return Error::new(ErrorKind::InvalidInput, msg);
}

fn read_json(data: String) -> serde_json::Result<()> {
    let v: serde_json::Value = serde_json::from_str(&data)?;
    println!("{:#?}", v["crypto"]["ciphertext"]);
    println!("{}", v["crypto"]["ciphertext"]);
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", usage_line(&args[0]));
        return Err(Error::new(ErrorKind::InvalidInput, "nope"));
    }
    
    let filename = &args[1];
    let contents = fs::read_to_string(filename)?;
    read_json(contents)?;
    Ok(())
}
