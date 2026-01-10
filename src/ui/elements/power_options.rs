use std::process::Command;

use gpui::{
    App, Corner, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
};

use crate::config::Config;
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
                    .hover(|style| style.bg(config.theme.muted))
                    .child(Icon::Power.build(config.theme.foreground)),
            )
            .content(|cx| {
                DropdownContent::new(cx)
                    .w_40()
                    .item(
                        "power-option-lock",
                        "Lock",
                        Some(Icon::Lock),
                        |window, _| match Command::new("loginctl").arg("lock-session").spawn() {
                            Ok(_) => window.remove_window(),
                            Err(e) => {
                                eprintln!("Failed to lock session: {}", e);
                            }
                        },
                    )
                    .item(
                        "power-option-sleep",
                        "Sleep",
                        Some(Icon::Sleep),
                        |window, _| match Command::new("systemctl").arg("suspend").spawn() {
                            Ok(_) => window.remove_window(),
                            Err(e) => {
                                eprintln!("Failed to sleep: {}", e);
                            }
                        },
                    )
                    .item(
                        "power-option-shut-down",
                        "Shut down",
                        Some(Icon::Power),
                        |window, _| match Command::new("systemctl").arg("poweroff").spawn() {
                            Ok(_) => window.remove_window(),
                            Err(e) => {
                                eprintln!("Failed to shut down: {}", e);
                            }
                        },
                    )
                    .item(
                        "power-option-restart",
                        "Restart",
                        Some(Icon::Restart),
                        |window, _| match Command::new("systemctl").arg("reboot").spawn() {
                            Ok(_) => window.remove_window(),
                            Err(e) => {
                                eprintln!("Failed to restart: {}", e);
                            }
                        },
                    )
            })
    }
}
