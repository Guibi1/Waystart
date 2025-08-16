pub mod client;
pub mod server;

const SOCKET_PATH: &str = "/tmp/waystart.sock";

const MESSAGE_SHOW: &[u8] = b"show";
const MESSAGE_HIDE: &[u8] = b"hide";
const MESSAGE_QUIT: &[u8] = b"quit";
