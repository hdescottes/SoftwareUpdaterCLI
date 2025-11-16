use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;


const FILE_PATH: &str = "apps.json";


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomApp {
    pub name: String,
    pub update_command: String,
    pub current_version_command: String,
    pub latest_version_command: String,
}


pub fn load_apps() -> Vec<CustomApp> {
    if !Path::new(FILE_PATH).exists() {
        return vec![];
    }

    let mut file = File::open(FILE_PATH).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}


pub fn save_apps(apps: &[CustomApp]) {
    let data = serde_json::to_string_pretty(apps).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}