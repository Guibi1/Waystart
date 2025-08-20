use gpui::{
    AppContext, Application, Bounds, Focusable, TitlebarOptions, WindowBounds, WindowDecorations,
    WindowKind, WindowOptions, px, size,
};

use crate::config::Config;
use crate::entries::SearchEntries;
use crate::ipc::client::{SocketClient, SocketMessage};
use crate::ipc::server::SocketServer;
use crate::ui::{CloseWaystart, Waystart};

mod cli;
mod config;
mod entries;
mod ipc;
mod ui;

fn main() {
    match cli::Waystart::from_env_or_exit().subcommand {
        cli::WaystartCmd::Standalone(_) => match SocketClient::try_connect().ok() {
            Some(client) => client.send_message_socket(SocketMessage::Show),
            None => {
                start_app(false);
            }
        },

        cli::WaystartCmd::Daemon(daemon) => {
            if daemon.exit {
                let client = SocketClient::connect();
                client.send_message_socket(SocketMessage::Quit);
            } else {
                start_app(true);
            }
        }

        cli::WaystartCmd::Show(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Show);
        }

        cli::WaystartCmd::Hide(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Hide);
        }
    }
}

fn start_app(daemonize: bool) {
    Application::new().with_assets(ui::Assets).run(move |cx| {
        ui::init(cx);
        cx.set_global(SearchEntries::load());
        cx.set_global(Config::load());
        cx.on_action(move |_: &CloseWaystart, cx| {
            if daemonize {
                cx.hide();
            } else {
                cx.quit();
            }
        });

        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    kind: WindowKind::PopUp,
                    is_movable: true,
                    show: !daemonize,
                    focus: !daemonize,
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_decorations: Some(WindowDecorations::Client),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Waystart".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let root = cx.new(Waystart::new);
                    window.focus(&root.focus_handle(cx));
                    root
                },
            )
            .unwrap();

        if daemonize {
            let server = SocketServer::new(cx.to_async(), window);
            server.listen();
        }
    });
}
