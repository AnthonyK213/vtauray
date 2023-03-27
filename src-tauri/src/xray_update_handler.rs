use std::{io::copy, path::PathBuf};
use crate::util::*;
use anyhow::Result;
use reqwest;
use std::fs::File;
use tauri::api::path::*;
use tauri::window::Window;

pub struct XrayUpdateHandler {}

impl XrayUpdateHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn update(&self, _: &Window) -> Result<()> {
        let url = "https://github.com/XTLS/Xray-core/releases/latest/download/Xray-linux-64.zip";
        if let Some(path) = config_dir() {
            let mut config_path_buf = PathBuf::from(path);
            config_path_buf.push("vtauray");
            log!("Config dir: {:?}", &config_path_buf);
            let response = reqwest::get(url).await?;
            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.zip");

                log!("File to download: {}", fname);
                let fname = config_path_buf.as_path().join(fname);
                log!("Will be located under: {:?}", fname);
                File::create(fname)?
            };
            let content = response.text().await?;
            copy(&mut content.as_bytes(), &mut dest)?;
        }
        Ok(())
    }
}
