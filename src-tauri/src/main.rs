#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod v2ray_handler;

use tauri::{
    CustomMenuItem, Manager, RunEvent, State, SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
    WindowEvent,
};
use v2ray_handler::V2rayHandler;

#[derive(Clone, serde::Serialize)]
struct Payload {
    m_type: u8,
    message: String,
}

#[tauri::command]
fn v2ray_connect(
    state: State<'_, V2rayHandler>,
    window: Window,
    path: String,
) -> Result<(), String> {
    state.start(window, path)
}

#[tauri::command]
fn v2ray_disconnect(state: State<'_, V2rayHandler>, window: Window) {
    state.stop_and_emit(&window);
}

fn main() {
    let v_handler = V2rayHandler::new();

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("connect".to_string(), "Connect"))
        .add_item(CustomMenuItem::new("disconnect".to_string(), "Disconnect"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(v_handler)
        .system_tray(system_tray)
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::DoubleClick { .. } => {
                app_handle.get_window("main").unwrap().show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let s: State<V2rayHandler> = app_handle.state();
                match id.as_str() {
                    "connect" => {},
                    "disconnect" => {
                        let _ = s.stop();
                    },
                    "quit" => {
                        let _ = s.stop();
                        app_handle.exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![v2ray_connect, v2ray_disconnect])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {}
            RunEvent::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CloseRequested { api, .. } => {
                    app_handle.get_window("main").unwrap().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            },
            _ => {}
        });
}
