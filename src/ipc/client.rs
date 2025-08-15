use std::cell::RefCell;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::ipc::{MESSAGE_CLOSE, MESSAGE_OPEN, MESSAGE_QUIT, SOCKET_PATH};

pub struct SocketClient {
    stream: RefCell<UnixStream>,
}

impl SocketClient {
    pub fn connect() -> Self {
        SocketClient {
            stream: RefCell::new(
                UnixStream::connect(SOCKET_PATH).expect("Failed to bind IPC socket"),
            ),
        }
    }

    pub fn send_message_socket(&self, message: SocketMessage) {
        let mut stream = self.stream.borrow_mut();
        if let Err(e) = match message {
            SocketMessage::Open => stream.write_all(MESSAGE_OPEN),
            SocketMessage::Close => stream.write_all(MESSAGE_CLOSE),
            SocketMessage::Quit => stream.write_all(MESSAGE_QUIT),
        } {
            eprintln!("Failed to send IPC message: {}", e);
        }
    }
}

pub enum SocketMessage {
    Open,
    Close,
    Quit,
}
