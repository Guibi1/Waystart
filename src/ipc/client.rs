use std::cell::RefCell;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::ipc::{MESSAGE_HIDE, MESSAGE_QUIT, MESSAGE_SHOW, SOCKET_PATH};

pub struct SocketClient {
    stream: RefCell<UnixStream>,
}

impl SocketClient {
    pub fn connect() -> Self {
        Self::try_connect().expect("Failed to bind IPC socket")
    }

    pub fn try_connect() -> std::io::Result<Self> {
        Ok(Self {
            stream: RefCell::new(UnixStream::connect(SOCKET_PATH)?),
        })
    }

    pub fn send_message_socket(&self, message: SocketMessage) {
        let mut stream = self.stream.borrow_mut();
        if let Err(e) = match message {
            SocketMessage::Show => stream.write_all(MESSAGE_SHOW),
            SocketMessage::Hide => stream.write_all(MESSAGE_HIDE),
            SocketMessage::Quit => stream.write_all(MESSAGE_QUIT),
        } {
            eprintln!("Failed to send IPC message: {}", e);
        }
    }
}

pub enum SocketMessage {
    Show,
    Hide,
    Quit,
}
