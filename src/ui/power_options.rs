use std::process::Command;

use gpui::{
    App, Corner, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
};

use crate::config::Config;
use crate::ui::CloseWaystart;
use crate::ui::elements::{Dropdown, DropdownContent, Icon};

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
                    .rounded_lg()
                    .hover(|style| style.bg(config.muted))
                    .child(Icon::Power.build(config.foreground)),
            )
            .content(|cx| {
                DropdownContent::new(cx)
                    .w_40()
                    .item(
                        "power-option-lock",
                        "Lock",
                        Some(Icon::Lock),
                        |_, cx| match Command::new("loginctl").arg("lock-session").spawn() {
                            Ok(_) => cx.dispatch_action(&CloseWaystart {}),
                            Err(e) => {
                                eprintln!("Failed to lock session: {}", e);
                            }
                        },
                    )
                    .item("power-option-sleep", "Sleep", Some(Icon::Sleep), |_, cx| {
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
                        Some(Icon::Power),
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
                        Some(Icon::Restart),
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
