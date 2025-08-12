use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window};

use crate::components::ui::PALETTE;

pub fn shortcut(text: impl Into<SharedString>) -> Shortcut {
    Shortcut { text: text.into() }
}

#[derive(IntoElement)]
pub struct Shortcut {
    text: SharedString,
}

impl RenderOnce for Shortcut {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        div()
            .text_sm()
            .px_1p5()
            .py_1()
            .line_height(px(12.))
            .rounded_sm()
            .bg(PALETTE.muted)
            .text_color(PALETTE.foreground)
            .child(self.text)
    }
}
