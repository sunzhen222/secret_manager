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
use std::io;
pub mod secret;

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

    println!("Input password:");
    let password = rpassword::read_password().unwrap();
    println!("password:{}", password);
    if query == "show" {
        print_all_secret(filename, password)?;
    } else if query == "edit"{
        let mut name = String::new();
        println!("Input name:");
        io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();
        let mut account = String::new();
        println!("Input account:");
        io::stdin().read_line(&mut account)?;
        account = account.trim().to_string();
        let mut secret = String::new();
        println!("Input secret:");
        io::stdin().read_line(&mut secret)?;
        secret = secret.trim().to_string();
        let secret_info: SecretInfo = SecretInfo {
            name: name,
            account: account,
            secret: secret,
        };
        edit_secret(secret_info, filename, password)?;
    } else if query == "remove"{
        let mut name = String::new();
        println!("Input name:");
        io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();
        remove_secret(filename, password, name)?;
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
    let plain_string = secret::decrypt(file_content, password);
    println!("{}", &plain_string);
    let v: Value = serde_json::from_str(&plain_string)?;
    let secret_info = v["secret_info"].to_string();
        let secret_info_vec: Vec<SecretInfo> = serde_json::from_str(&secret_info)?;
    for item in secret_info_vec {
        println!("{},{},{}", item.name, item.account, item.secret);
    }
    Ok(())
}


fn remove_secret (
    filename: String,
    password: String,
    name: String,
) -> Result<()> {
    let file_content = fs::read(&filename)?;
    if file_content.len() == 0 {
        return Err(Error::msg("The file is empty."));
    }
    let plain_string = secret::decrypt(file_content, password.clone());
    let v: Value = serde_json::from_str(&plain_string)?;
    let secret_info = v["secret_info"].to_string();
    let mut secret_info_vec: Vec<SecretInfo> = serde_json::from_str(&secret_info)?;

    //find if the item that has a same name already exists.
    if let Some(index) = secret_info_vec.iter().position(|x| x.name == name) {
        secret_info_vec.remove(index);
    } else {
        return Err(Error::msg("not found a same name."));
    }
    let personal_info = json!({
        "secret_info": secret_info_vec,
    });
    let json_string = serde_json::to_string(&personal_info)?;
    println!("json_string JSON: {}", json_string);
    let encrypt_data = secret::encrypt(json_string, password);
    fs::write(filename, encrypt_data)?;
    Ok(())
}
