use anyhow::Error;
use serde::{Serialize, Deserialize};
//use serde_json::Result;
use serde_json;
use serde_json::json;
use serde_json::Value;
use anyhow::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
pub mod secret;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct SecretInfo {
    name: String,
    account: String,
    secret: String,
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = args[2].to_string();

    println!("query:{}", query);
    println!("filename:{}", filename);
    println!("In file {}", filename);

    let mut password: String = String::new();
    println!("Input password:");
    io::stdin().read_line(&mut password)?;
    println!("password:{}", password);
    if query == "show" {
        print_all_secret(filename, password)?;
    } else if query == "edit"{
        let secret_info1: SecretInfo = SecretInfo {
            name: "google".to_string(),
            account: "jy070342".to_string(),
            secret: "82621".to_string(),
        };
        edit_secret(secret_info1, filename, "test password".to_string())?;
    }

    Ok(())
}


fn edit_secret(
    secret_info: SecretInfo,
    filename: String,
    password: String,
) -> Result<()> {
    if Path::new(&filename).is_file() == false {
        println!("{} not exist", filename);
        File::create(&filename)?;
    }
    let file_content = fs::read(&filename)?;
    let mut secret_info_vec: Vec<SecretInfo> = Vec::new();
    if file_content.len() == 0 {
        println!("empty file");
        secret_info_vec.push(secret_info);
        let personal_info = json!({
            "secret_info": secret_info_vec,
        });
        let json_string = serde_json::to_string(&personal_info)?;
        println!("json_string JSON: {}", json_string);
        let encrypt_data = secret::encrypt(json_string, password);
        fs::write(filename, encrypt_data)?;
    } else {
        let plain_string = secret::decrypt(file_content, password.clone());
        println!("plain_string={}", plain_string);
        let v: Value = serde_json::from_str(&plain_string)?;
        let secret_info_base = v["secret_info"].to_string();
        let mut secret_info_vec: Vec<SecretInfo> = serde_json::from_str(&secret_info_base)?;

        //find if the item that has a same name already exists.
        if let Some(index) = secret_info_vec.iter().position(|x| x.name == secret_info.name) {
            secret_info_vec.remove(index);
        } else {
            println!("not found a same name");
        }
        secret_info_vec.push(secret_info);
        let personal_info = json!({
            "secret_info": secret_info_vec,
        });
        let json_string = serde_json::to_string(&personal_info)?;
        println!("json_string JSON: {}", json_string);
        let encrypt_data = secret::encrypt(json_string, password);
        fs::write(filename, encrypt_data)?;
    }
    Ok(())
}


fn print_all_secret (
    filename: String,
    password: String,
) -> Result<()> {
    let file_content = fs::read(&filename)?;
    if file_content.len() == 0 {
        return Err(Error::msg("The file is empty."));
    }

    Ok(())
}
