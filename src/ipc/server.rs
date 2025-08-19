use smol::io::{AsyncBufReadExt, BufReader};
use smol::net::unix::{UnixListener, UnixStream};
use smol::stream::StreamExt;

use gpui::{AsyncApp, WindowHandle};

use crate::ipc::{MESSAGE_HIDE, MESSAGE_QUIT, MESSAGE_SHOW, SOCKET_PATH};
use crate::ui::Waystart;

#[derive(Clone)]
pub struct SocketServer {
    app: AsyncApp,
    window: WindowHandle<Waystart>,
}

impl SocketServer {
    pub fn new(app: AsyncApp, window: WindowHandle<Waystart>) -> Self {
        Self { app, window }
    }

    pub fn listen(&self) {
        if std::fs::exists(SOCKET_PATH).ok().unwrap_or(false) {
            std::fs::remove_file(SOCKET_PATH).expect("Failed to remove existing IPC socket");
        }

        let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind IPC socket");

        let this = self.clone();
        self.app
            .spawn(async move |cx| {
                loop {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            cx.spawn(async move |cx| {
                                Self::handle_ipc_stream(stream, this.window, cx).await
                            })
                            .detach();
                        }
                        Err(e) => {
                            eprintln!("Failed to accept IPC connection: {}", e);
                        }
                    }
                }
            })
            .detach();
    }

    async fn handle_ipc_stream(
        stream: UnixStream,
        window: WindowHandle<Waystart>,
        cx: &mut AsyncApp,
    ) {
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        while let Some(Ok(message)) = lines.next().await {
            if let Err(e) = match message.as_bytes() {
                MESSAGE_SHOW => window.update(cx, |waystart, window, cx| {
                    waystart.reset_search(cx);
                    window.activate_window();
                }),
                MESSAGE_HIDE => cx.update(|cx| cx.hide()),
                MESSAGE_QUIT => cx.update(|cx| cx.quit()),
                _ => {
                    eprintln!("Received unknown IPC message: {}", message);
                    return;
                }
            } {
                eprintln!("Lost reference to app: {}", e);
                return;
            }
        }
    }
}
