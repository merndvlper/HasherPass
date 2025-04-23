use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::crypto;

const FILE_PATH: &str = "passwords.json";

pub fn add_entry(name: &str, key: &str) {
    let mut data = load_data();
    println!("Enter Password:");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let encrypted = crypto::encrypt(&password.trim(), key);
    data.insert(name.to_string(), encrypted);
    save_data(&data);
    println!("✔ '{}' saved.", name);
}

pub fn get_entry(name: &str, key: &str) {
    let data = load_data();
    if let Some(enc) = data.get(name) {
        if let Ok(decrypted) = crypto::decrypt(enc, key) {
            println!("🔑 Password: {}", decrypted);
        } else {
            println!("⚠️ password didn't encrypt.");
        }
    } else {
        println!("❌ '{}' not found.", name);
    }
}

pub fn list_entries() {
    let data = load_data();
    println!("📋 listed passwords:");
    for key in data.keys() {
        println!("- {}", key);
    }
}

pub fn delete_entry(name: &str) {
    let mut data = load_data();
    if data.remove(name).is_some() {
        save_data(&data);
        println!("🗑️ '{}' removed.", name);
    } else {
        println!("❌ '{}' not found.", name);
    }
}

pub fn load_data() -> HashMap<String, String> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

pub fn save_data(data: &HashMap<String, String>) {
    let json = serde_json::to_string_pretty(data).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
