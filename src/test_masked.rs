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

fn mask_string(s: &str, visible: usize) -> String {
    if s.len() <= visible {
        "*".repeat(s.len())
    } else {
        format!(
            "{}{}",
            &s[..visible],
            "*".repeat(s.len() - visible)
        )
    }
}

fn mask_ip(ip: &str) -> String {
    let mut parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        parts[2] = "***";
        parts[3] = "***";
    }
    parts.join(".")
}

fn main() {
    let data = fs::read_to_string("src/data.json").expect("Failed to read file");

    let records: Vec<Record> =
        serde_json::from_str(&data).expect("Failed to parse JSON");

    let masked_records: Vec<Record> = records
        .into_iter()
        .map(|r| Record {
            full_name: r.full_name,
            email: mask_string(&r.email, 3),
            phone: mask_string(&r.phone, 4),
            id_number: mask_string(&r.id_number, 2),
            tax_id: mask_string(&r.tax_id, 2),
            ip: mask_ip(&r.ip),
        })
        .collect();

    let output = serde_json::to_string_pretty(&masked_records).unwrap();
    println!("{}", output);
}



