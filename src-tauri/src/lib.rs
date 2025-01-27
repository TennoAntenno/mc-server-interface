use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};

use reqwest::Error as ReqwestError;
use serde_json::Value;
use thiserror::Error;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::process::Child;

use tauri::Window;
use tauri::Emitter;

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
const SAVE_PATH: &str = "../server";

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
        let mut file = File::create(Path::new(SAVE_PATH).join(&file_name)).map_err(DownloadError::Io)?;
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


#[tauri::command]
async fn watch_latest_log(window: Window) -> Result<(), String> {
    use tokio::time::{self, Duration};

    println!("Starting to monitor log file!");

    let log_file_path = format!("{}/logs/latest.log", SAVE_PATH);

    // spawn an asynchronous task to handle periodic file reading
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1)); // check every second
        let mut last_content = String::new();

        loop {
            interval.tick().await; // wait for the next tick

            match fs::read_to_string(&log_file_path) {
                Ok(content) => {
                    // emit only if the content has changed
                    if content != last_content {
                        last_content = content.clone();

                        // emit the content to the frontend
                        if let Err(err) = window.emit("log-updated", content) {
                            eprintln!("Failed to emit log content: {}", err);
                        } else {
                            println!("Log content sent to frontend");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read log file: {}", err);
                }
            }
        }
    });

    Ok(())
}

lazy_static! {
    static ref SERVER_PROCESS: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
async fn open_paper_server() -> Result<(), String> {
    let entries = fs::read_dir(SAVE_PATH).map_err(|e| format!("Failed to read save path: {}", e))?;

    let paper_jar = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                file_name.starts_with("paper-") && file_name.ends_with(".jar")
            } else {
                false
            }
        })
        .ok_or_else(|| "No Paper server JAR file found in the save path.".to_string())?;

    let mut process = Command::new("java")
        .arg("-jar")
        .arg(&paper_jar)
        .arg("--nogui")
        .current_dir(SAVE_PATH)
        .stdin(Stdio::piped()) // pipe stdin for command input
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("Failed to execute the JAR file: {}", e))?;

    {
        let mut server_process = SERVER_PROCESS.lock().unwrap();
        *server_process = Some(process);
    }

    println!("Paper server started successfully.");
    Ok(())
}

#[tauri::command]
async fn restart_paper_server() -> Result<(), String> {
    // run stop command then start command
    run_command("stop".to_string()).await?;
    open_paper_server().await?;
    Ok(())
}

#[tauri::command]
async fn run_command(command: String) -> Result<(), String> {
    let mut server_process = SERVER_PROCESS.lock().unwrap();
    if let Some(process) = server_process.as_mut() {
        if let Some(stdin) = process.stdin.as_mut() {
            // write the command to the stdin
            stdin.write_all(command.as_bytes()).map_err(|e| format!("Failed to write to server stdin: {}", e))?;
            stdin.write_all(b"\n").map_err(|e| format!("Failed to write newline to server stdin: {}", e))?;
            println!("Command sent: {}", command);
            Ok(())
        } else {
            Err("Server stdin is not available.".to_string())
        }
    } else {
        Err("No server process found. Start the server first.".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_paper_server,
            open_paper_server,
            restart_paper_server,
            watch_latest_log,
            run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
