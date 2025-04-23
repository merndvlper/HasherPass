# 🔐 HasherPass – A Simple and Secure Password Manager

**HasherPass** is a command-line password manager built with Rust. It allows you to securely store, retrieve, and manage your passwords in an encrypted local JSON file.

---

## ✨ Features

- 🔐 **Add**, **retrieve**, **list**, and **delete** passwords from the CLI.
- 🧩 Modular structure: `crypto` and `storage` components.
- 🔒 Uses `SHA-256` and optional master key to encrypt stored data.
- 🗃️ Stores encrypted data in a local `data.json` file.

---

## 🛠 Technologies Used

| Tech        | Purpose                          |
|-------------|----------------------------------|
| Rust        | Core programming language        |
| Clap        | Command-line argument parsing    |
| Serde       | JSON serialization/deserialization |
| Tokio       | Async runtime for Axum           |
| SHA2 crate  | Cryptographic hashing            |

---

## 🚀 Getting Started

### ✅ Prerequisites

- Rust (latest stable version) — [Install Rust](https://rustup.rs)
- Cargo (comes with Rust)

### 📦 CLI Commands

   ```bash
    cargo run -- add <name>      # Add a new password
    cargo run -- get <name>      # Retrieve a stored password
    cargo run -- list            # List all saved password entries
    cargo run -- delete <name>   # Delete an entry by name
 ```

 ###📁 Data Storage
 
   ```bash
 {
  "Password Tag 1": "ENCRYPTED_DATA",
  "Password Tag 2": "ENCRYPTED_DATA"
 }
 ```
