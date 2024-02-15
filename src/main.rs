use serde::{Serialize, Deserialize};
//use serde_json::Result;
use serde_json;
use serde_json::json;
//use serde_json::Value;
use anyhow::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
pub mod secret;

#[derive(Serialize, Deserialize, Debug)]
struct SecretInfo {
    name: String,
    account: String,
    secret: String,
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let filename = args[2].to_string();
    let query = &args[1];

    println!("query:{}", query);
    println!("filename:{}", filename);
    println!("In file {}", filename);

    let secret_info1: SecretInfo = SecretInfo {
        name: "google".to_string(),
        account: "jy70342".to_string(),
        secret: "8262".to_string(),
    };
    edit_secret(secret_info1, filename, "test password".to_string())?;

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
        let plain_string = secret::decrypt(file_content, "test password".to_string());
        println!("plain_string={}", plain_string);
    }
    Ok(())
}
