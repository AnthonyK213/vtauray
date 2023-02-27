use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    m_type: u8,
    message: String,
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
