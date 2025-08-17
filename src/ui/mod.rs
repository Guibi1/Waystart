use gpui::App;

pub mod elements;
mod power_options;
mod waystart;

pub use power_options::PowerOptions;
pub use waystart::{Close as CloseWaystart, Waystart};

pub fn init(cx: &mut App) {
    waystart::init(cx);
    elements::init(cx);
}
