use std::borrow::Cow;

use gpui::{App, AssetSource};
use rust_embed::RustEmbed;

pub mod elements;
mod power_options;
mod waystart;

pub use power_options::PowerOptions;
pub use waystart::{Close as CloseWaystart, Waystart};

pub fn init(cx: &mut App) {
    waystart::init(cx);
    elements::init(cx);
}

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
