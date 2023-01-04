#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod v2ray_handler;

use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Write},
};
use tauri::{Manager, RunEvent, State, Window};
use v2ray_handler::V2rayHandler;

#[derive(Clone, serde::Serialize)]
struct Payload {
    m_type: u8,
    message: String,
}

#[tauri::command]
fn v2ray_connect(state: State<'_, V2rayHandler>, window: Window, path: String) {
    state.start(window, path);
}

#[tauri::command]
fn v2ray_disconnect(state: State<'_, V2rayHandler>, window: Window) {
    state.stop_and_emit(&window);
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    let mut buf = String::new();
    let mapper = |e: io::Error| e.to_string();
    let mut file = fs::File::open(path).map_err(mapper)?;
    let _ = file.read_to_string(&mut buf).map_err(mapper)?;
    Ok(buf)
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    let mapper = |e: io::Error| e.to_string();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create_new(!fs::metadata(&path).is_ok())
        .open(path)
        .map_err(mapper)?;
    file.write_all(content.as_bytes()).map_err(mapper)?;
    Ok(())
}

fn main() {
    let v_handler = V2rayHandler::new();

    tauri::Builder::default()
        .manage(v_handler)
        .invoke_handler(tauri::generate_handler![
            v2ray_connect,
            v2ray_disconnect,
            read_file,
            write_file
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {}
            RunEvent::ExitRequested { .. } => {
                let s: State<V2rayHandler> = app_handle.state();
                let _ = s.stop();
            }
            _ => {}
        });
}
