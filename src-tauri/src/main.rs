#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod v2ray_handler;

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

fn main() {
    let v_handler = V2rayHandler::new();

    tauri::Builder::default()
        .manage(v_handler)
        .invoke_handler(tauri::generate_handler![v2ray_connect, v2ray_disconnect])
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
