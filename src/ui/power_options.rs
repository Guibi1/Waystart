use gpui::{
    div, App, Corner, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window,
};

use crate::ui::elements::dropdown::{Dropdown, DropdownContent};
use crate::ui::palette::PALETTE;

#[derive(IntoElement)]
pub struct PowerOptions {}

impl PowerOptions {
    pub fn new() -> Self {
        PowerOptions {}
    }
}

impl RenderOnce for PowerOptions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Dropdown::new("power-options")
            .anchor(Corner::TopRight)
            .trigger(
                div()
                    .size_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_lg()
                    .text_color(PALETTE.foreground)
                    .rounded_lg()
                    .hover(|style| style.bg(PALETTE.muted).text_color(PALETTE.muted_foreground))
                    .child("⏻"),
            )
            .content(|cx| {
                DropdownContent::new(cx)
                    .w_32()
                    .item("power-option-lock", "Lock", |_, _| {
                        println!("Lock action triggered");
                    })
                    .item("power-option-sleep", "Sleep", |_, _| {})
                    .item("power-option-shut-down", "Shut down", |_, _| {})
                    .item("power-option-restart", "Restart", |_, _| {})
            })
    }
}
