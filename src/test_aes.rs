use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    full_name: String,
    email: String,
    phone: String,
    id_number: String,
    tax_id: String,
    ip: String,
}

fn encrypt_field(key: &Key<Aes256Gcm>, plaintext: &str) -> String {
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failure!");
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);

    general_purpose::STANDARD.encode(combined)
}

fn main() {
    let key = Key::<Aes256Gcm>::from_slice(b"anexampleverysecurekey1234567890!!");

    let data = fs::read_to_string("src/data.json").expect("Failed to read file");

    let records: Vec<Record> =
        serde_json::from_str(&data).expect("Failed to parse JSON");

    let encrypted_records: Vec<Record> = records
        .into_iter()
        .map(|r| Record {
            full_name: r.full_name,
            email: encrypt_field(key, &r.email),
            phone: encrypt_field(key, &r.phone),
            id_number: encrypt_field(key, &r.id_number),
            tax_id: encrypt_field(key, &r.tax_id),
            ip: encrypt_field(key, &r.ip),
        })
        .collect();

    let output = serde_json::to_string_pretty(&encrypted_records).unwrap();

    fs::write("src/encrypted_data.json", output).expect("Failed to write output");

    println!("[+] Done. Output saved to `src/encrypted_data.json`");
}
