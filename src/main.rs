use gpui::{
    px, size, AppContext, Application, BackgroundExecutor, Bounds, Focusable, TitlebarOptions,
    WindowBounds, WindowDecorations, WindowKind, WindowOptions,
};

use crate::ipc::client::{SocketClient, SocketMessage};
use crate::ipc::server::SocketServer;
use crate::ui::Waystart;

mod cli;
mod desktop_entry;
mod ipc;
mod ui;

fn main() {
    match cli::Waystart::from_env_or_exit().subcommand {
        cli::WaystartCmd::Daemon(daemon) => {
            if daemon.exit {
                let client = SocketClient::connect();
                client.send_message_socket(SocketMessage::Quit);
            } else {
                let executor = start_app();
                let socket_listener = SocketServer::new(&executor);

                executor
                    .spawn(async move {
                        socket_listener.listen();
                    })
                    .detach();
            }
        }

        cli::WaystartCmd::Show(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Open);
        }

        cli::WaystartCmd::Hide(_) => {
            let client = SocketClient::connect();
            client.send_message_socket(SocketMessage::Close);
        }
    }
}

fn start_app() -> BackgroundExecutor {
    let desktop_entries = desktop_entry::get_desktop_entries();
    let application = Application::new();
    let executor = application.background_executor();

    application.run(|cx| {
        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        ui::init(cx);

        cx.open_window(
            WindowOptions {
                kind: WindowKind::PopUp,
                focus: true,
                show: false,
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
                let root = cx.new(|cx| Waystart::new(desktop_entries, cx));
                window.focus(&root.focus_handle(cx));
                root
            },
        )
        .unwrap();

        cx.hide();
    });

    executor
}
