use std::fmt::Display;
use tauri::Window;
use BandWidth::*;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    m_type: u8,
    message: String,
}

#[derive(Clone, serde::Serialize)]
pub struct StatsPayload {
    pub outbound_proxy_traffic_downlink_speed: String,
}

pub enum BandWidth {
    B(usize),
    KB(f32),
    MB(f32),
}

impl Display for BandWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            B(m) => write!(f, "{} B/s", m),
            KB(m) => write!(f, "{:.1} KB/s", m),
            MB(m) => write!(f, "{:.1} MB/s", m),
        }
    }
}

pub fn escape_html(string: String) -> String {
    string
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#039;")
}

pub fn emit_logging(window: &Window, m_type: u8, message: String) -> tauri::Result<()> {
    window.emit(
        "v-logging",
        Payload {
            m_type,
            message: escape_html(message),
        },
    )
}

pub fn emit_stats(window: &Window, stats_payload: StatsPayload) -> tauri::Result<()> {
    window.emit("v-stats", stats_payload)
}

pub fn get_available_port() -> Option<u16> {
    // (8000..9000).find(|port| match TcpListener::bind(("127.0.0.1", *port)) {
    // Ok(_) => true,
    // Err(_) => false,
    // })
    Some(49723)
}

pub fn bandwitdh_display(amount_in_bit: i64, interval_in_ms: u64) -> Option<BandWidth> {
    if amount_in_bit < 0 || interval_in_ms <= 0 {
        None
    } else {
        let speed = amount_in_bit as f32 * 0.1220703125 / interval_in_ms as f32;
        if speed < 900.0 {
            Some(KB(speed))
        } else {
            Some(MB(speed / 1024.0))
        }
    }
}

macro_rules! log {
    () => {
        #[cfg(debug_assertions)]
        println!("--------------------------------");
    };
    ($arg:tt) => {
        #[cfg(debug_assertions)]
        println!("[{}]: {}", chrono::offset::Local::now(), $arg);
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[{}]: {}", chrono::offset::Local::now(), format!($($arg)*));
    };
}

pub(crate) use log;
