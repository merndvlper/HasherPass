use sha2::{Digest, Sha256};
use base64::{engine::general_purpose, Engine as _};
use rpassword::read_password;

pub fn get_master_key() -> String {
    println!("🔐 Enter the master key");
    let password = read_password().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}

pub fn encrypt(data: &str, key: &str) -> String {
    let encoded = format!("{}:{}", key, data);
    general_purpose::STANDARD.encode(encoded)
}

pub fn decrypt(encoded: &str, key: &str) -> Result<String, &'static str> {
    let decoded = general_purpose::STANDARD.decode(encoded).map_err(|_| "Decode error")?;
    let decoded_str = String::from_utf8(decoded).map_err(|_| "UTF8 error")?;
    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
    if parts.len() == 2 && parts[0] == key {
        Ok(parts[1].to_string())
    } else {
        Err("Key wrong!")
    }
}
