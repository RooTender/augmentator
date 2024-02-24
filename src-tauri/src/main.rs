// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn your_tauri_command(options: Vec<String>) -> Result<String, String> {
    // Handle the options as needed, e.g., process them, save them, etc.
    println!("Received options: {:?}", options);

    // Return a success message or result
    Ok("Dataset created successfully".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![your_tauri_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
