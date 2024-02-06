use serde::{Serialize, Deserialize};
use serde_json::Result;
use serde_json::json;

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
    // 创建一个 Person 结构体实例
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
    //let personal_info: PersonalInfo = PersonalInfo {
    //    secret_info: secret_info1,
    //}

    let secret_array = [&secret_info1, &secret_info2];

    let json_string = serde_json::to_string(&secret_info1)?;
    println!("Serialized JSON: {}", json_string);

    let json_array_string = serde_json::to_string(&secret_array)?;
    println!("json_array_string JSON: {}", json_array_string);

    //let json_psn_string = serde_json::to_string(&personal_info1)?;
    //println!("json_array_string JSON: {}", json_array_string);

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
