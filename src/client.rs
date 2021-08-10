use serde::{Serialize, Deserialize};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::stream::Stream;
use reqwest::blocking;
use reqwest::header::{AUTHORIZATION, USER_AGENT, HeaderMap};
use crate::thread_pool::ThreadPool;
use crate::consts::{API_URL, PERMISSIONS};

#[derive(Serialize, Deserialize, Debug)]
struct SessStartLimit {
    total: u16,
    remaining: u16,
    reset_after: u16,
    max_concurrency: u16
}

#[derive(Serialize, Deserialize, Debug)]
struct GateReq {
    url: String,
    shards: u16,
    session_start_limit: SessStartLimit,
}

#[allow(dead_code)]
pub struct Client {
    header_map: HeaderMap,
    session_id: String,
    beat_interval: u32,
    token: String,
    intents: u16,
    ws_address: String,
    ws_socket: Option<WebSocket<AutoStream>>,
    gateway_meta: GateReq,
}

impl Client {

    pub fn new() -> Self {
        let key = env!("DISC_BOT_TOKEN");

        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.insert(AUTHORIZATION, format!("Bot {}", key).parse().unwrap());
        header_map.insert(USER_AGENT, "DiscordBot (\"www.github.com/sarahgreywolf/greybot\")".parse().unwrap());

        let req_client = blocking::Client::new();
        let response: GateReq = req_client.get(format!("{}gateway/bot", API_URL))
            .headers(header_map.clone())
            .send().unwrap().json().unwrap();
        
        println!("{:?}", response);

        Self {
            header_map,
            session_id: "".to_string(),
            beat_interval: 0,
            token: key.to_string(),
            intents: 0,
            ws_address: response.url.clone(),
            ws_socket: None,
            gateway_meta: response,
        }
    }

    fn login() {
        
    }

}
