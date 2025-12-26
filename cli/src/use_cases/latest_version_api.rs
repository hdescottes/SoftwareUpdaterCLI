use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Release {
    version: String,
}

type Response = HashMap<String, Vec<Release>>;

pub async fn retrieve_latest_version(url: &str) -> String {
    fetch_latest_version(url).await.unwrap_or_else(|e| {
        eprintln!("Erreur : {}", e);
        "unknown".to_string()
    })
}

async fn fetch_latest_version(url: &str) -> Result<String, Box<dyn Error>> {
    let data: Response = reqwest::get(url).await?.json().await?;

    let version = data
        .values()
        .flat_map(|releases| releases.iter())
        .find_map(|r| Some(r.version.clone()))
        .ok_or("version introuvable")?;

    Ok(version)
}