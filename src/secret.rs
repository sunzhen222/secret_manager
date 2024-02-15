use sha2::{Sha512, Digest};
use std::fmt::Write;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn get_plain(cipher: Vec<u8>, password: String) -> String {
    let mut hash = Sha512::new();
    hash.update(password.as_bytes());
    let result = hash.finalize();
    let (first_half, second_half) = result.split_at(32);
    let key: Vec<u8> = first_half.to_vec();
    let iv: Vec<u8> = second_half[0..16].to_vec();

    //let mut ori_data = Vec::new();
    //let mut index = 0;
    //loop {
    //    ori_data.push(index);
    //    index += 1;
    //    if index >= 32 {
    //        break;
    //    }
    //}
    //let ori_data = "hello".as_bytes();
    let string: String = key.iter().map(|&byte| format!("{:02X}", byte)).collect();
    println!("key={}", string);
    let string: String = iv.iter().map(|&byte| format!("{:02X}", byte)).collect();
    println!("iv={}", string);

    //let cipher = encrypt_aes_cbc(&ori_data, &key, &iv);
    let string: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    println!("cipher={}", string);
    let plain = decrypt_aes_cbc(&cipher, &key, &iv);
    let string: String = plain.iter().map(|&byte| format!("{:02X}", byte)).collect();
    println!("plain={}", string);

    //let cipher_str: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("cipher={}", cipher_str);
    //let plain = decrypt_aes_cbc(&cipher, &key, &iv);
    //let plain: String = plain.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("plain={}", plain);

    //let first_vec: String = first_vec.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("first_vec={}", first_vec);
    //let second_vec: String = second_vec.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("second_vec={}", second_vec);

    //let mut hex_string = String::new();
    //write!(&mut hex_string, "{:x}", result).expect("Failed to write hex string");
    //println!("result={}", result.len());
    //println!("hex_string={}", hex_string);

    //let cipher: String = cipher.iter().map(|&byte| format!("{:02X}", byte)).collect();
    //println!("cipher={}", cipher);

    "123".to_string()
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

