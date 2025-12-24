use crate::domain::custom_app::CustomApp;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn load_apps(file_path: &Path) -> Vec<CustomApp> {
    if !Path::new(file_path).exists() {
        return vec![];
    }

    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

pub fn save_apps(apps: &[CustomApp], file_path: &Path) {
    let data = serde_json::to_string_pretty(apps).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}