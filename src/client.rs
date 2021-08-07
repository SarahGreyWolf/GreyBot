use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::stream::Stream;
use reqwest::blocking;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::io::{Read, Write};
use crate::thread_pool::ThreadPool;
use crate::consts::API_URL;

pub struct Client {
    session_id: String,
    beat_interval: u32,
    token: String,
    intents: u16,
    ws_socket: Option<WebSocket<AutoStream>>,
}

impl Client {

    pub fn new() {
        let key = env!("DISC_BOT_TOKEN");

        let req_client = blocking::Client::new();
        let response = req_client.get(format!("{}gateway/bot", API_URL))
            .header(AUTHORIZATION, format!("Bot {}", key))
            .header(USER_AGENT, "DiscordBot (\"www.github.com/sarahgreywolf/greybot\", \"1.0.0\")")
            .send().unwrap().text().unwrap();

        println!("{:?}", response);
    }

}
