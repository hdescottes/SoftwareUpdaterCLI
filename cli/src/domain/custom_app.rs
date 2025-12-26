use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomApp {
    pub name: String,
    pub update_command: String,
    pub current_version_path: String,
    pub latest_version_url: String,
}