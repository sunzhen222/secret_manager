use sha2::{Sha512, Digest};
//use std::fmt::Write;
//use std::ptr::null;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
//use hex_literal::hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(plain: String, password: String) -> Vec<u8> {
    let mut hash = Sha512::new();
    hash.update(password.as_bytes());
    let result = hash.finalize();
    let (first_half, second_half) = result.split_at(32);
    let key: Vec<u8> = first_half.to_vec();
    let iv: Vec<u8> = second_half[0..16].to_vec();

    let cipher = encrypt_aes_cbc(plain.as_bytes(), &key, &iv);
    //let string: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("cipher={}", string);

    cipher
}

pub fn decrypt(cipher: Vec<u8>, password: String) -> String {
    let mut hash = Sha512::new();
    hash.update(password.as_bytes());
    let result = hash.finalize();
    let (first_half, second_half) = result.split_at(32);
    let key: Vec<u8> = first_half.to_vec();
    let iv: Vec<u8> = second_half[0..16].to_vec();

    //let string: String = key.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("key={}", string);
    //let string: String = iv.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("iv={}", string);

    //let cipher = encrypt_aes_cbc(&ori_data, &key, &iv);
    //let string: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("cipher={}", string);
    let plain = decrypt_aes_cbc(&cipher, &key, &iv);

    //let plain = decrypt_aes_cbc(&cipher, &key, &iv);
    //let plain: String = plain.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("plain={}", plain);

    String::from_utf8(plain).unwrap()
}


fn decrypt_aes_cbc(buf: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let block_cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let plain = block_cipher.decrypt_vec(buf).unwrap();
    plain
}

fn encrypt_aes_cbc(buf: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let block_cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let cipher = block_cipher.encrypt_vec(buf);
    cipher
}

