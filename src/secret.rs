use sha2::{Sha512, Digest};
//use hex::ToHex;
use std::fmt::Write;


pub fn get_plain(cipher: Vec<u8>, password: String) -> String {
//pub fn get_plain() {
    let mut hash = Sha512::new();
    hash.update(password.as_bytes());
    let result = hash.finalize();
    let mut hex_string = String::new();
    write!(&mut hex_string, "{:x}", result).expect("Failed to write hex string");
    println!("result={}", result.len());
    //println!("hex_string={}", hex_string);

    let cipher: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    println!("cipher={}", cipher);

    hex_string
}
