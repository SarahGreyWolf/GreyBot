use tungstenite::protocol::WebSocket;
use tungstenite::stream::Stream;
use crate::thread_pool::ThreadPool;

pub struct Client<S> {
    session_id: String,
    beat_interval: u32,
    token: String,
    intents: u16,
    ws_socket: Option<WebSocket<S>>,
}
