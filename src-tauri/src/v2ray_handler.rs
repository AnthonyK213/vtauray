use std::{
    io::Read,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};
use tauri::window::Window;

use crate::Payload;

pub struct V2rayHandler {
    process: Arc<Mutex<Option<Child>>>,
}

fn escape_html(string: String) -> String {
    string
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#039;")
}

fn emit_logging(window: &Window, m_type: u8, message: String) -> tauri::Result<()> {
    window.emit(
        "v-logging",
        Payload {
            m_type,
            message: escape_html(message),
        },
    )
}

impl V2rayHandler {
    pub fn new() -> Self {
        V2rayHandler {
            process: Arc::new(Mutex::new(None)),
        }
    }

    // pub fn load() {}

    pub fn start(&self, window: Window, path: String) {
        let mut lock = self.process.lock().unwrap();

        if !lock.is_none() {
            let _ = lock.take().unwrap().kill();
        }

        let mut p = Command::new("./xray/xray")
            .arg("-config")
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to launch v2ray");

        let mut child_out = p.stdout.take().unwrap();
        let mut child_err = p.stderr.take().unwrap();

        *lock = Some(p);

        let win_out = window.clone();
        let mut buf_out = [0; 1024];

        thread::spawn(move || loop {
            let mut size_out = child_out.read(&mut buf_out).expect("Stdout failure");

            if size_out == 0 {
                #[cfg(debug_assertions)]
                let _ = emit_logging(&win_out, 1, "Stdout stopped".into());
                break;
            } else if size_out > buf_out.len() {
                size_out = buf_out.len();
            }

            let buf_str: String = String::from_utf8_lossy(&buf_out[0..size_out]).into();

            let _ = emit_logging(
                &win_out,
                if buf_str.contains("[Warning]") { 1 } else { 0 },
                buf_str,
            );
        });

        let win_err = window.clone();
        let mut buf_err = [0; 1024];

        thread::spawn(move || loop {
            let mut size_err = child_err.read(&mut buf_err).expect("Stderr failure");

            if size_err == 0 {
                #[cfg(debug_assertions)]
                let _ = emit_logging(&win_err, 1, "Stderr stopped".into());
                break;
            } else if size_err > buf_err.len() {
                size_err = buf_err.len();
            }

            let _ = emit_logging(
                &win_err,
                2,
                String::from_utf8_lossy(&buf_err[0..size_err]).into(),
            );
        });
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();

        if lock.is_none() {
            Err("Xray has already stopped".into())
        } else {
            lock.take().unwrap().kill().map_err(|e| e.to_string())
        }
    }

    pub fn stop_and_emit(&self, window: &Window) {
        match self.stop() {
            Ok(()) => {
                let _ = emit_logging(window, 1, "Xray stopped".into());
            }
            Err(err) => {
                let _ = emit_logging(window, 2, err);
            }
        }
    }
}
