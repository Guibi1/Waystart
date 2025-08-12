use gpui::App;

pub mod power_options;
pub mod ui;

pub fn init(cx: &mut App) {
    ui::init(cx);
}
