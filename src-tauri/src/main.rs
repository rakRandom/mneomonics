// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
use auth::authenticate_user;

#[tauri::command]
async fn login(email: String, password: String) -> Result<String, String> {
    match authenticate_user(&email, &password).await {
        Ok(token) => Ok(token),
        Err(e) => Err(format!("Falha na autenticação: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
