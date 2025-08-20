use gpui::App;

pub mod dropdown;
pub mod icon;
pub mod input;
pub mod separator;
pub mod shortcut;

pub use dropdown::{Dropdown, DropdownContent};
pub use icon::Icon;
pub use input::TextInput;
pub use separator::Separator;
pub use shortcut::Shortcut;

pub(super) fn init(cx: &mut App) {
    input::init(cx);
    dropdown::init(cx);
}
