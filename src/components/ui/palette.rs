use std::sync::LazyLock;

use gpui::{rgb, Rgba};

pub static PALETTE: LazyLock<Palette> = LazyLock::new(|| Palette::default());

pub struct Palette {
    pub background: Rgba,
    pub foreground: Rgba,
    pub muted: Rgba,
    pub muted_foreground: Rgba,
    pub border: Rgba,
    pub accent: Rgba,
    pub accent_foreground: Rgba,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            background: rgb(0x1e1e2e),
            foreground: rgb(0xcdd6f4),
            muted: rgb(0x313244),
            muted_foreground: rgb(0xbac2de),
            border: rgb(0x45475a),
            accent: rgb(0xcba6f7),
            accent_foreground: rgb(0x1e1e2e),
        }
    }
}
