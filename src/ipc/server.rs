use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};

use gpui::BackgroundExecutor;

use crate::ipc::{MESSAGE_CLOSE, MESSAGE_OPEN, MESSAGE_QUIT, SOCKET_PATH};

pub struct SocketServer {
    executor: BackgroundExecutor,
}

impl SocketServer {
    pub fn new(executor: &BackgroundExecutor) -> Self {
        SocketServer {
            executor: executor.clone(),
        }
    }

    pub fn listen(&self) {
        let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind IPC socket");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.executor
                        .spawn(async move {
                            Self::handle_ipc_stream(stream);
                        })
                        .detach();
                }
                Err(e) => {
                    eprintln!("Failed to accept IPC connection: {}", e);
                }
            }
        }
    }

    fn handle_ipc_stream(stream: UnixStream) {
        let reader = BufReader::new(stream);

        for message in reader.lines() {
            let Ok(message) = message else {
                break;
            };

            match message.as_bytes() {
                MESSAGE_OPEN => {
                    // Handle open command
                }
                MESSAGE_CLOSE => {
                    // Handle close command
                }
                MESSAGE_QUIT => {
                    // Handle quit command
                }
                _ => {
                    eprintln!("Received unknown IPC message: {}", message);
                }
            }
        }
    }
}
