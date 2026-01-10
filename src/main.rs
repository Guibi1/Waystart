use gpui::{
    App, AppContext, Application, BorrowAppContext, Bounds, Entity, Focusable, QuitMode,
    TitlebarOptions, WindowBounds, WindowDecorations, WindowHandle, WindowKind, WindowOptions,
    point, px, size,
};

use crate::config::Config;
use crate::finder::Finder;
use crate::finder::desktop::SearchEntries;
use crate::ipc::client::{SocketClient, SocketMessage};
use crate::ipc::server::SocketServer;
use crate::ui::Waystart;

mod cli;
mod config;
mod finder;
mod ipc;
mod ui;

fn main() {
    let daemon = match cli::Waystart::from_env_or_exit().subcommand {
        cli::WaystartCmd::Standalone(_) => {
            if let Ok(client) = SocketClient::try_connect() {
                client.send_message_socket(SocketMessage::Open);
                return;
            }
            // Start without daemon
            false
        }
        cli::WaystartCmd::Daemon(daemon) => {
            if daemon.exit {
                let client = SocketClient::connect();
                client.send_message_socket(SocketMessage::Quit);
                return;
            }
            // Start with daemon
            true
        }
        cli::WaystartCmd::Open(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Open);
            return;
        }
        cli::WaystartCmd::Close(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Close);
            return;
        }
        cli::WaystartCmd::Toggle(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Toggle);
            return;
        }
    };

    Application::new()
        .with_assets(ui::Assets)
        .with_quit_mode(if daemon {
            QuitMode::Explicit
        } else {
            QuitMode::LastWindowClosed
        })
        .run(move |cx| {
            ui::init(cx);
            cx.set_global(SearchEntries::new());
            cx.set_global(Config::load());

            cx.on_app_quit(|cx| {
                let entries = cx.remove_global::<SearchEntries>();
                async move { entries.save().await }
            })
            .detach();

            let waystart = cx.new(Waystart::new);

            if daemon {
                let server = SocketServer::new(cx.to_async(), waystart);
                server.listen();
            } else {
                open_window(cx, waystart);
            }
        });
}

pub fn open_window(cx: &mut App, waystart: Entity<Waystart>) -> WindowHandle<Waystart> {
    cx.update_global(|entries: &mut SearchEntries, _| entries.sort_by_frequency());
    cx.update_entity(&waystart, |waystart: &mut Waystart, cx| {
        waystart.reset_search(cx)
    });

    let bounds = Bounds::centered(None, size(px(800.), px(500.)), cx);
    cx.open_window(
        WindowOptions {
            #[cfg(target_os = "linux")]
            kind: WindowKind::LayerShell(gpui::layer_shell::LayerShellOptions {
                namespace: "waystart".to_string(),
                anchor: gpui::layer_shell::Anchor::LEFT
                    | gpui::layer_shell::Anchor::RIGHT
                    | gpui::layer_shell::Anchor::BOTTOM
                    | gpui::layer_shell::Anchor::TOP,
                margin: Some((px(0.), px(0.), px(40.), px(0.))),
                keyboard_interactivity: gpui::layer_shell::KeyboardInteractivity::Exclusive,
                ..Default::default()
            }),
            #[cfg(not(target_os = "linux"))]
            kind: WindowKind::PopUp,
            is_resizable: false,
            is_minimizable: false,
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_decorations: Some(WindowDecorations::Client),
            titlebar: Some(TitlebarOptions {
                title: Some("Waystart".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(-100.0), px(0.0))),
            }),
            ..Default::default()
        },
        |window, cx| {
            window.focus(&waystart.focus_handle(cx), cx);
            waystart
        },
    )
    .unwrap()
}
