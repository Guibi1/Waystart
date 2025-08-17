use std::{env, path::PathBuf};

use gpui::{Global, Rgba, rgb};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub background: Rgba,
    pub foreground: Rgba,
    pub border: Rgba,
    pub muted: Rgba,
    pub muted_foreground: Rgba,
    pub accent: Rgba,
    pub accent_foreground: Rgba,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: rgb(0x1e1e2e),
            foreground: rgb(0xcdd6f4),
            border: rgb(0x45475a),
            muted: rgb(0x313244),
            muted_foreground: rgb(0xbac2de),
            accent: rgb(0xcba6f7),
            accent_foreground: rgb(0x1e1e2e),
        }
    }
}

impl Global for Config {}

impl Config {
    pub fn load() -> Self {
        match std::fs::read_to_string(Self::get_path()) {
            Ok(file) => toml::from_str(&file).expect("Failed to parse config.toml"),
            Err(_) => Self::default(),
        }
    }

    fn get_path() -> std::path::PathBuf {
        match env::var("XDG_CONFIG_DIR") {
            Ok(config) => PathBuf::from(config).join("waystart.toml"),
            Err(_) => env::home_dir().unwrap().join(".config/waystart.toml"),
        }
    }
}
