extern crate crypto;
use crate::data_process::Data;
use crypto::scrypt::ScryptParams;
use crypto::scrypt::scrypt;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use crate::utilities::bytes_to_hexstring;

pub fn derive_key(data: &Data) -> Result<Vec<u8>, &'static str> {
    let mut n = data.n as f64;
    n = n.log2();
    let log_2_n = n as u8;
    let params = ScryptParams::new(log_2_n, data.r, data.p);
    // Alternative:
    // let mut result: [u8; 32] = [0; 32];
    // or more properly:
    // let mut result: [u8; data.dklen as usize] = [0; data.dklen as usize];
    let mut result: Vec<u8> = vec![0; data.dklen as usize];
    scrypt(&(data.password), &(data.salt), &params, &mut result);
    Ok(result.to_vec())
}

pub fn check_key(data: &Data, key: &Vec<u8>) -> bool {
    let mut k: Vec<u8> = key.split_at(16).0.to_vec();
    for el in data.ct.iter() {
        k.push(*el);
    }
    // ----------------------------------
    // NOT RIGHT, HERE 
    println!("{:?}", k);

    // create a SHA3-256 object
    let mut hasher = Sha3::keccak256();

    // write input message
    hasher.input(&k);

    // read hash digest
    let hex = hasher.result_str();
    println!("{}", hex);
    println!("{}", bytes_to_hexstring(&data.mac));
    return false;    
}
