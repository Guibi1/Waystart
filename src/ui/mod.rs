use gpui::App;

mod palette;
mod power_options;
mod waystart;

pub mod elements;
pub use palette::PALETTE;
pub use power_options::PowerOptions;
pub use waystart::{Close as CloseWaystart, Waystart};

pub fn init(cx: &mut App) {
    waystart::init(cx);
    elements::init(cx);
}
