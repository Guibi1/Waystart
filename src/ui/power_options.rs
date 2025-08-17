use std::process::Command;

use gpui::{
    App, Corner, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
};

use crate::config::Config;
use crate::ui::CloseWaystart;
use crate::ui::elements::dropdown::{Dropdown, DropdownContent};

#[derive(IntoElement)]
pub struct PowerOptions {}

impl PowerOptions {
    pub fn new() -> Self {
        PowerOptions {}
    }
}

impl RenderOnce for PowerOptions {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let config = cx.global::<Config>();

        Dropdown::new("power-options")
            .anchor(Corner::TopRight)
            .trigger(
                div()
                    .size_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_lg()
                    .text_color(config.foreground)
                    .rounded_lg()
                    .hover(|style| style.bg(config.muted).text_color(config.muted_foreground))
                    .child("⏻"),
            )
            .content(|cx| {
                DropdownContent::new(cx)
                    .w_32()
                    .item("power-option-lock", "Lock", |_, cx| {
                        match Command::new("loginctl").arg("lock-session").spawn() {
                            Ok(_) => cx.dispatch_action(&CloseWaystart {}),
                            Err(e) => {
                                eprintln!("Failed to lock session: {}", e);
                            }
                        }
                    })
                    .item("power-option-sleep", "Sleep", |_, cx| {
                        match Command::new("systemctl").arg("suspend").spawn() {
                            Ok(_) => cx.dispatch_action(&CloseWaystart {}),
                            Err(e) => {
                                eprintln!("Failed to sleep: {}", e);
                            }
                        }
                    })
                    .item(
                        "power-option-shut-down",
                        "Shut down",
                        |_, cx| match Command::new("systemctl").arg("poweroff").spawn() {
                            Ok(_) => cx.dispatch_action(&CloseWaystart {}),
                            Err(e) => {
                                eprintln!("Failed to shut down: {}", e);
                            }
                        },
                    )
                    .item(
                        "power-option-restart",
                        "Restart",
                        |_, cx| match Command::new("systemctl").arg("reboot").spawn() {
                            Ok(_) => cx.dispatch_action(&CloseWaystart {}),
                            Err(e) => {
                                eprintln!("Failed to restart: {}", e);
                            }
                        },
                    )
            })
    }
}
