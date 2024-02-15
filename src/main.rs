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

#[derive(Serialize, Deserialize, Debug)]
struct SecretInfo {
    name: String,
    account: String,
    secret: String,
}

struct PersonalInfo {
    secret_info: Vec<SecretInfo>,
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let filename = &args[2];
    let query = &args[1];

    println!("query:{}", query);
    println!("filename:{}", filename);
    println!("In file {}", filename);

    if Path::new(filename).is_file() == false {
        println!("{} not exist", filename);
        File::create(filename)?;
    }
    let file_content = fs::read(filename)?;
    if file_content.len() == 0 {
        println!("empty file");
    } else {
        secret::get_plain(file_content, "test password".to_string());
    }
    let secret_info1 = SecretInfo {
        name: "google".to_string(),
        account: "jy70342".to_string(),
        secret: "8262".to_string(),
    };
    let secret_info2 = SecretInfo {
        name: "google".to_string(),
        account: "jy70342".to_string(),
        secret: "8262".to_string(),
    };

    //let secret_array = [secret_info1, secret_info2];
//
    //let json_string = serde_json::to_string(&secret_info1)?;
    //println!("Serialized JSON: {}", json_string);

    //let personal_info = PersonalInfo {
    //    secret_info: vec![secret_info1, secret_info2],
    //};
    //let personal_info = PersonalInfo {
    //    secret_info: vec![
    //        SecretInfo {
    //            name: "John".to_string(),
    //            account: "john@example.com".to_string(),
    //            secret: "password123".to_string(),
    //        },
    //        SecretInfo {
    //            name: "Alice".to_string(),
    //            account: "alice@example.com".to_string(),
    //            secret: "securepassword".to_string(),
    //        },
    //    ],
    //};
    let personal_info = json!({
        "secret_info": vec![secret_info1, secret_info2],
    });


    let json_string = serde_json::to_string(&personal_info)?;
    println!("json_string JSON: {}", json_string);

    let v: Value = serde_json::from_str(&json_string)?;
    let secret_info = v["secret_info"].to_string();
    //println!("v: {}", secret_info);
    let secret_info: Vec<SecretInfo> = serde_json::from_str(&secret_info)?;
    println!("name: {}", secret_info[0].name);

    //edit_secret(&secret_info1, "123".to_string());

    Ok(())
}



fn edit_secret(
    secret_info: &SecretInfo,
    password: String,
) {
    println!("name={}", secret_info.name);
    println!("account={}", secret_info.account);
    println!("secret={}", secret_info.secret);
    println!("password={}", password);
}
