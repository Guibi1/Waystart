use gpui::App;

mod desktop_entry;
mod power_options;
mod waystart;

pub mod ui;
pub use desktop_entry::DesktopEntry;
pub use power_options::PowerOptions;
pub use waystart::Waystart;

pub fn init(cx: &mut App) {
    waystart::init(cx);
    ui::init(cx);
}
