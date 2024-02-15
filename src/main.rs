use serde::{Serialize, Deserialize};
//use serde_json::Result;
use serde_json;
use anyhow::Result;
use std::env;
use std::fs;
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

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let filename = &args[2];
    let query = &args[1];

    println!("query:{}", query);
    println!("filename:{}", filename);
    println!("In file {}", filename);

    let file_content = fs::read(filename)?;
    secret::get_plain(file_content, "test password".to_string());

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

    let secret_array = [&secret_info1, &secret_info2];

    let json_string = serde_json::to_string(&secret_info1)?;
    println!("Serialized JSON: {}", json_string);

    let json_array_string = serde_json::to_string(&secret_array)?;
    println!("json_array_string JSON: {}", json_array_string);

    let deserialized_info: SecretInfo = serde_json::from_str(&json_string)?;
    println!("Deserialized Person: {:?}", deserialized_info);

    edit_secret(&secret_info1, "123".to_string());

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
