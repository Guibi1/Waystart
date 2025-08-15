use gpui::App;

mod desktop_entry;
mod palette;
mod power_options;
mod waystart;

pub mod elements;
pub use desktop_entry::DesktopEntry;
pub use palette::PALETTE;
pub use power_options::PowerOptions;
pub use waystart::Waystart;

pub fn init(cx: &mut App) {
    waystart::init(cx);
    elements::init(cx);
}
