use gpui::App;

pub mod dropdown;
pub mod input;
pub mod palette;
pub mod separator;
pub mod shortcut;

pub use input::TextInput;
pub use palette::PALETTE;
pub use separator::Separator;
pub use shortcut::Shortcut;

pub(super) fn init(cx: &mut App) {
    input::init(cx);
    dropdown::init(cx);
}
