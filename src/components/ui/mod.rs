use gpui::App;

pub mod dropdown;
pub mod input;
pub mod palette;
pub mod separator;
pub mod shortcut;

pub use input::*;
pub use palette::*;
pub use separator::separator;
pub use shortcut::*;

pub fn init(cx: &mut App) {
    input::init(cx);
    dropdown::init(cx);
}
