use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window};

use crate::components::ui::PALETTE;

#[derive(IntoElement)]
pub struct Shortcut {
    text: SharedString,
}

impl Shortcut {
    pub fn new(text: impl Into<SharedString>) -> Shortcut {
        Shortcut { text: text.into() }
    }
}

impl RenderOnce for Shortcut {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        div()
            .text_sm()
            .px_1p5()
            .py_1()
            .line_height(px(12.))
            .rounded_sm()
            .bg(PALETTE.accent)
            .text_color(PALETTE.accent_foreground)
            .child(self.text)
    }
}
