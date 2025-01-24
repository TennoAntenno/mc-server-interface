use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use reqwest::Error as ReqwestError;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
enum DownloadError {
    #[error("Reqwest error")]
    Reqwest(#[from] ReqwestError),
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Missing data: {0}")]
    MissingData(String),
}

// Convert `DownloadError` to a `String` for Tauri IPC compatibility.
impl From<DownloadError> for String {
    fn from(error: DownloadError) -> Self {
        error.to_string()
    }
}

const API_BASE_URL: &str = "https://api.papermc.io/v2/projects/paper";

#[tauri::command]
async fn get_paper_server() -> Result<(), String> {
    println!("Fetching the latest Paper server version...");

    // fetch the latest version
    let version_response: Value = reqwest::get(API_BASE_URL)
        .await
        .map_err(DownloadError::Reqwest)?
        .json()
        .await
        .map_err(DownloadError::Reqwest)?;

    let latest_version = version_response["versions"]
        .as_array()
        .and_then(|versions| versions.last())
        .and_then(|v| v.as_str())
        .ok_or_else(|| DownloadError::MissingData("Failed to find the latest version".to_string()))?;

    println!("Latest version: {}", latest_version);

    // fetch the latest build for that version
    let builds_url = format!("{API_BASE_URL}/versions/{latest_version}");
    let builds_response: Value = reqwest::get(&builds_url)
        .await
        .map_err(DownloadError::Reqwest)?
        .json()
        .await
        .map_err(DownloadError::Reqwest)?;

    let latest_build = builds_response["builds"]
        .as_array()
        .and_then(|builds| builds.last())
        .and_then(|b| b.as_u64())
        .ok_or_else(|| DownloadError::MissingData("Failed to find the latest build".to_string()))?;

    println!("Latest build: {}", latest_build);

    // fetch the download url
    let download_url = format!(
        "{API_BASE_URL}/versions/{latest_version}/builds/{latest_build}/downloads/paper-{latest_version}-{latest_build}.jar"
    );

    println!("Downloading Paper server from: {}", download_url);

    // download the server jar file
    let response = reqwest::get(&download_url).await.map_err(DownloadError::Reqwest)?;
    if response.status().is_success() {
        let file_name = format!("paper-{latest_version}-{latest_build}.jar");
        let mut file = File::create(Path::new(&file_name)).map_err(DownloadError::Io)?;
        let content = response.bytes().await.map_err(DownloadError::Reqwest)?;
        file.write_all(&content).map_err(DownloadError::Io)?;
        println!("Paper server downloaded and saved as '{}'.", file_name);
    } else {
        return Err(DownloadError::MissingData(format!(
            "Failed to download the Paper server. HTTP status: {}",
            response.status()
        ))
        .into());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_paper_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
