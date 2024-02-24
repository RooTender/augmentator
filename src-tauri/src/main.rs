// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Directories {
    input: String,
    target: String,
    output: String,
}

#[tauri::command]
fn augment_dataset(directories: Directories, transformations: Vec<String>) -> Result<String, String> {
    let mut missing_directories = Vec::new();

    if directories.input.trim().is_empty() {
        missing_directories.push("input");
    }
    if directories.target.trim().is_empty() {
        missing_directories.push("target");
    }
    if directories.output.trim().is_empty() {
        missing_directories.push("output");
    }

    if !missing_directories.is_empty() {
        let missing_directories_str = missing_directories.join(", ");
        return Err(format!("Directories {} aren't set.", missing_directories_str));
    }

    println!("Received directories: {:?}", directories);
    println!("Received transformations: {:?}", transformations);

    Ok("Dataset augmentation process started successfully.".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![augment_dataset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
