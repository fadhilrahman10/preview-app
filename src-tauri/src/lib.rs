// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use opencv::{
    prelude::*,
    videoio,
    imgcodecs,
    core,
};
use base64::{engine::general_purpose, Engine as _};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_camera_frame() -> Result<String, String> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).map_err(|e| e.to_string())?;
    if !videoio::VideoCapture::is_opened(&cam).map_err(|e| e.to_string())? {
        return Err("Camera not available".into());
    }

    let mut frame = Mat::default();
    cam.read(&mut frame).map_err(|e| e.to_string())?;
    if frame.size().map_err(|e| e.to_string())?.width == 0 {
        return Err("Empty frame".into());
    }

    let mut buffer = core::Vector::new();
    imgcodecs::imencode(".jpg", &frame, &mut buffer, &core::Vector::new())
        .map_err(|e| e.to_string())?;

    Ok(format!("data:image/jpeg;base64,{}", general_purpose::STANDARD.encode(&buffer)))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_camera_frame])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
