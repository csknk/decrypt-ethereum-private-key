# Decrypt Ethereum Private Key
Rust project that decrypts an Ethereum keyfile to recover the original private key.

I recently built the same functionality in a Python project, and wanted to see how to create the same functionality in Rust.

__Be careful with your private keys__. If you use this repo to decrypt your private key from an Ethereum keyfile and a malicious person gets hold of it, they gain control over the funds held by that private key.

The objective of this project is to replicate a key component of any wallet - decrypting the private key so that it can be used to sign transactions.

If you want to make a backup of Ethereum keys, just backup the keyfiles - the private key is encrypted already, and any Ethereum client should be able to use the keyfile format. This assumes of course that you have used a strong passphrase to secure your keys. 

Table of Contents
-----------------
* [Introduction](#introduction)
* [Generate an Ethereum Keyfile](#generate-an-ethereum-keyfile)
* [Usage](#usage)
* [Encryption of Keys in Ethereum](#encryption-of-keys-in-ethereum)
* [Key Derivation](#key-derivation)
* [Verify Password by Message Authentication](#verify-password-by-message-authentication)
* [Decryption](#decryption)
* [Dependencies](#dependencies)
* [References](#references)

Introduction
------------
In cryptocurrencies like Bitcoin and Ethereum, private keys define ownership of assets on a public blockchain. As such, it is vitally important that such keys are not exposed - access to private keys is synonymous with access to funds.

For this reason, private keys are generally encrypted before being stored.

### Bitcoin
In the case of the Bitcoin Core client (the original cryptocurrency client), private keys are stored in an internal database. By default, this is named `wallet.dat` and located in the `wallets` subdirectory of the Bitcoin data directory. The wallet file is a Berkeley DB file that contains keys and related transactions. The wallet file is not a text file and is not human-readable, and users have the choice whether or not to encrypt the wallet.

Wallet encryption involves encrypting the private keys with a random master key which is in turn symmetrically encrypted using a key derived from passphrase - [full description of the relevant encryption protocols][11]. Keys are decrypted only when necessary, either by GUI prompt or by means of the `walletpassphrase` command.

An encrypted wallet file is fairly tightly coupled to the Bitcoin Core client - you need the core client to parse the wallet. However, Bitcoin Core provides an option for exporting private keys by means of the `dumpprivkey` CLI command - keys might then be imported into other wallet software.

### Ethereum
Ethereum keyfiles are defined in the [Web3 Secret Storage Definition][17]. Communication with nodes takes place through RPC calls to a JSON-RPC interface, which uses JSON as a data format.

[Go Ethereum][5] is the official Golang implementation of the Ethereum protocol. It's CLI client, `geth`, does not allow private keys to be exported in plaintext. This is in contrast to Bitcoin Core where the `dumpprivkey` command provides access to decrypted private keys.

The Ethereum approach is interesting in that keyfiles contain information relating to their decryption. You could easily print an Ethereum keyfile and have a paper-backup of the key, the security of which is determined by your passphrase.

Ethereum keyfiles are JSON text files that are comprised of a symmetrically encrypted private key along with additional metadata relating to the encryption scheme.

Keyfiles are stored by default in a `keystore` directory, and are human readable. Each keyfile provides the encrypted key, along with the metadata required to decrypt it.

Generate an Ethereum Keyfile
----------------------------
For testing purposes, you need a private key and an associated keyfile (which contains the encrypted private key). Geth can be used for this.

Geth will generate a keyfile from a supplied private key, which should be 32 bytes long expressed as a hex string:

```bash
# cd into a temporary working directory
cd $(mktemp -d)

# Make a private key from 32 pseudo-random bytes
head -c 32 /dev/random | xxd -ps -c 32 > plain_key.txt

# Make an Ethereum key file - assumes geth is installed, prompts for password
geth --datadir . account import plain_key.txt
```
The sample keyfile shown below is generated from a private key `82633960e2a725ab641067a12b05fcaeca860d45ba785f634318490261e5d1a1` - 32 pseudo-random bytes - encrypted with the password "password123":

```json
{
  "address": "15d5d89632dc2d185aa27907ad42b1012ef1c982",
  "crypto": {
    "cipher": "aes-128-ctr",
    "ciphertext": "050d93d6a4e396a0cb74d021d0de9b1ed7860c0fd843b28acefbd3dc61314a19",
    "cipherparams": {
      "iv": "6aa1de28f8f43a522e6ac987c18bf66e"
    },
    "kdf": "scrypt",
    "kdfparams": {
      "dklen": 32,
      "n": 262144,
      "p": 1,
      "r": 8,
      "salt": "b04dcccf351dba67460e5bf322493ab25b4e1b314df970503ed43c392166d4c8"
    },
    "mac": "c9a7a0c880289d267c49bf828ace98ecb89c64d600bbeed718dac9f605083e61"
  },
  "id": "62b2bcce-9ba7-49a4-8f67-59fb366ac7dd",
  "version": 3
}

```
This keyfile is included in this repo: [UTC--2019-12-17T09-17-16.419911545Z--15d5d89632dc2d185aa27907ad42b1012ef1c982][15].

The correct private key is also provided in [correct-result][16].

Usage
-----
* Clone this repo, `cd` into the project root.
* Run `cargo run` with a path to an Ethereum keyfile as the first command-line argument - test file in project root.
* Enter your password when prompted - for the test file, the password is `password123`.
* If the password is correct, the private key will be output to stdout.
* If testing against the supplied test keyfile, check the result against [correct-result][16].

### Tests
Run `cargo test` to test the key derivation, authentication and decryption functions.

Encryption of Keys in Ethereum
------------------------------
The keyfile holds the encrypted private key in the `crypto.ciphertext` field.

The encryption scheme is an AES-128-CTR cipher, using scrypt as a key derivation function (to derive a block cipher key from a text-based password) and message authentication code (MAC) to authenticate the password.

The private key is symmetrically encrypted using AES-128 with block cipher mode CTR. In this case, the [scrypt][6] key derivation function is used to generate an AES symmetric key from the original password. An initialization vector is also required for decryption - and this is held in the `crypto.cipherparams.iv` field.

Relevant fields are:

* `crypto.cipher`: Denotes the cryptographic block-cipher algorithm, key size in bits and block cipher mode of operation.
* `crypto.ciphertext`: The encrypted private key.
* `crypto.cipherparams.iv`: The initialization vector required for AES in counter (CTR) mode.
* `crypto.kdf`: Denotes the key derivation function used - in this case, `scrypt`.
* `crypto.kdfparams`: These variables are used in the kdf function - see [decrypt.rs][8], [scrypt wikipedia][6]
* `crypto.mac`: Message authentication code - used to check the authenticity of the key derived from the user-supplied password.

Key Derivation: Scrypt
----------------------
Requires the user-supplied password and the `crypto.kdfparams`.

Uses `crypto::scrypt::scrypt` function from the [rust-crypto crate][18]

From [decrypt.rs][8]: 
```rust
/**
 * Derive key for decryption by means of scrypt with the provided parameters from the original
 * keyfile along with the user-supplied password (`data.password`).
 * */
pub fn derive_key(data: &Data) -> Result<Vec<u8>, &'static str> {
    let mut n = data.n as f64;
    n = n.log2();
    let log_2_n = n as u8;
    let params = ScryptParams::new(log_2_n, data.r, data.p);
    let mut result: Vec<u8> = vec![0; data.dklen as usize];
    scrypt(&data.password, &data.salt, &params, &mut result);
    Ok(result.to_vec())
}
```
__NOTE: this implementation of scrypt is slow__.

Verify Password by Message Authentication
-----------------------------------------
Once the key has been derived from the password, it is authenticated by:

* Taking the second-leftmost 16 bytes from the derived key.
* Concatenating this value (key excluding first 16 bytes) with the ciphertext bytes.
* Comparing the SHA-3 (keccak-256) hash of this value with the value of the `crypto.mac` field.
* If these values are the same, the key is authentic.

If the values do not match, the supplied password is incorrect.

Decryption
----------
Once the encryption key has been derived (and authenticated) from the user-supplied password and the KDF parameters, it can be used to decrypt `crypto.cipertext` - yielding the decrypted private key. 

Note that for AES 128 bit counter mode the aes_key must be 16 bytes, but the Ethereum keyfile key derivation algorithm uses scrypt to derive a 32 byte key from a user-supplied password.

The web3 protocol requires using the first 16 bytes of the derived key to AES decrypt ciphertext in AES-128 counter mode.

```rust

pub fn decrypt(data: &Data, key: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    use aes_ctr::Aes128Ctr;
    use aes_ctr::stream_cipher::generic_array::GenericArray;
    use aes_ctr::stream_cipher::{
        NewStreamCipher, SyncStreamCipher
    };
    let aes_key = GenericArray::from_slice(key.split_at(16).0);
    let initialization_vector = GenericArray::from_slice(&data.iv);
    let mut ct = (data.ct).clone();
    let mut cipher = Aes128Ctr::new(&aes_key, &initialization_vector);
    cipher.apply_keystream(&mut ct);
    Ok(ct.to_vec())
}
```

Dependencies
------------
Project developed on Ubuntu 16.04.

Requires `rustc`: if `cargo` is installed, move into the project repo and run `cargo run` or `cargo build`.

References
----------
* [Good description of Ethereum wallet encryption][10]
* [Bitcoin wallet encryption][11]
* [Pysha3][1] - SHA-3 wrapper(keccak) for Python
* [Keccak code package][2]
* [Keccak hashing: SHA-3][14]
* [Useful Stack Exchange answer][3]
* [Bitcoin Core dumpprivkey command][4]
* [Go Ethereum][5], GitHub repo
* [scrypt key derivation function][6]
* [Create an AES cipher using Python Crypto.Cipher.AES][12]
* [Creeat a counter function using Python Crypto.Util.Counter][13]


[1]: https://pypi.org/project/pysha3/
[2]: https://github.com/XKCP/XKCP
[3]: https://ethereum.stackexchange.com/questions/3720/how-do-i-get-the-raw-private-key-from-my-mist-keystore-file
[4]: https://bitcoin.org/en/developer-reference#dumpprivkey
[5]: https://github.com/ethereum/go-ethereum
[6]: https://en.wikipedia.org/wiki/Scrypt
[7]: /password_verify.py
[8]: /src/decrypt.rs#L12
[9]: https://pycryptodome.readthedocs.io/en/latest/src/cipher/cipher.html
[10]: https://cryptobook.nakov.com/symmetric-key-ciphers/ethereum-wallet-encryption
[11]: https://en.bitcoin.it/wiki/Wallet_encryption
[12]: https://pythonhosted.org/pycrypto/Crypto.Cipher.AES-module.html#new
[13]: https://pythonhosted.org/pycrypto/Crypto.Util.Counter-module.html
[14]: https://en.wikipedia.org/wiki/SHA-3
[15]: /UTC--2019-12-17T09-17-16.419911545Z--15d5d89632dc2d185aa27907ad42b1012ef1c982
[16]: /correct-result
[17]: https://github.com/ethereum/wiki/wiki/Web3-Secret-Storage-Definition
[18]: https://docs.rs/rust-crypto/0.2.36/crypto/scrypt/index.html
