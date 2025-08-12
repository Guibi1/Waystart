use gpui::{
    div, Context, Corner, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
};

use crate::components::ui::dropdown::{Dropdown, DropdownContent};
use crate::components::ui::palette::PALETTE;

pub struct PowerOptions {}

impl PowerOptions {
    pub fn new() -> Self {
        PowerOptions {}
    }
}

impl Render for PowerOptions {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<PowerOptions>,
    ) -> impl IntoElement {
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
