#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod statistics_handler;
mod util;
mod v2ray_handler;

use statistics_handler::StatisticsHandler;
use tauri::{
    CustomMenuItem, Manager, RunEvent, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent,
};
// use util::EmptyPayload;
use v2ray_handler::V2rayHandler;

#[tauri::command]
async fn v2ray_connect(
    v_h: State<'_, V2rayHandler>,
    s_h: State<'_, StatisticsHandler>,
    window: Window,
    path: String,
) -> Result<(), String> {
    v_h.start(&window, path)?;
    s_h.start(&window).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn v2ray_disconnect(
    v_h: State<'_, V2rayHandler>,
    s_h: State<'_, StatisticsHandler>,
    window: Window,
) -> Result<(), String> {
    v_h.stop_and_emit(&window);
    s_h.stop().await.map_err(|e| e.to_string())
}

fn main() {
    let v_handler = V2rayHandler::new();
    let s_handler = StatisticsHandler::new();

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("connect".to_string(), "Connect"))
        .add_item(CustomMenuItem::new("disconnect".to_string(), "Disconnect"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(v_handler)
        .manage(s_handler)
        .system_tray(system_tray)
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::DoubleClick { .. } => {
                app_handle.get_window("main").unwrap().show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // let v: State<V2rayHandler> = app_handle.state();
                // let s: State<StatisticsHandler> = app_handle.state();
                match id.as_str() {
                    "show" => {
                        app_handle.get_window("main").unwrap().show().unwrap();
                    }
                    "connect" => {
                        if let Some(window) = app_handle.get_window("main") {
                            let _ = window.emit("v-connect", 0);
                        }
                    }
                    "disconnect" => {
                        if let Some(window) = app_handle.get_window("main") {
                            let _ = window.emit("v-disconnect", 0);
                        }
                    }
                    "quit" => {
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
            RunEvent::ExitRequested { .. } => {
                let _ = app_handle.state::<StatisticsHandler>().stop();
                let _ = app_handle.state::<V2rayHandler>().stop();
            }
            _ => {}
        });
}
