use gpui::{div, px, App, IntoElement, RenderOnce, Styled, Window};

use crate::components::ui::PALETTE;

#[derive(IntoElement)]
pub struct Separator {}

impl Separator {
    pub fn new() -> Self {
        Separator {}
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        div().w_full().h(px(1.)).bg(PALETTE.muted)
    }
}
