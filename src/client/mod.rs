use crate::constants;
use crate::intents::Intents;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use reqwest::{self, Url};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Map, Number, Value};
use std::{fs, io::Write, sync::Arc, time::Duration};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use native_tls;
type EventHandlerList = Vec<Arc<Mutex<dyn EventHandler + Send + Sync + 'static>>>;

#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn on_Ready(&self, msg: String) {}
    async fn on_increased(&mut self, amount: i32) {}
    async fn on_decreased(&mut self, amount: i32) {}
}
impl EventHandler for Client {}

#[async_trait]
pub trait EventEmitter {
    async fn square(&self, msg: String) {}
    async fn increase(&mut self, amount: i32) {}
    async fn decrease(&mut self, amount: i32) {}
}
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatAck,
    Unknown(u8),
}
impl From<u8> for Operation {
    fn from(op: u8) -> Self {
        use Operation::*;
        match op {
            0 => Dispatch,
            1 => Heartbeat,
            2 => Identify,
            3 => PresenceUpdate,
            4 => VoiceStateUpdate,
            6 => Resume,
            7 => Reconnect,
            8 => RequestGuildMembers,
            9 => InvalidSession,
            10 => Hello,
            11 => HeartbeatAck,
            n => Unknown(n),
        }
    }
}

impl From<Operation> for u8 {
    fn from(op: Operation) -> Self {
        use Operation::*;
        match op {
            Dispatch => 0,
            Heartbeat => 1,
            Identify => 2,
            PresenceUpdate => 3,
            VoiceStateUpdate => 4,
            Resume => 6,
            Reconnect => 7,
            RequestGuildMembers => 8,
            InvalidSession => 9,
            Hello => 10,
            HeartbeatAck => 11,
            Unknown(n) => n,
        }
    }
}

impl Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'a> Deserialize<'a> for Operation {
    fn deserialize<D>(deserializer: D) -> Result<Operation, D::Error>
    where
        D: Deserializer<'a>,
    {
        Ok(u8::deserialize(deserializer)?.into())
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    pub op: Operation,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
}
#[async_trait]
impl EventEmitter for Client {
    async fn square(&self, msg: String) {
        for handler in self.event_handler.iter() {
            handler.lock().await.on_Ready(msg.clone()).await;
        }

        // self.on_squared().await;
    }
    async fn increase(&mut self, amount: i32) {
        for handler in self.event_handler.iter() {
            handler.lock().await.on_increased(amount).await;
        }

        // self.on_increased(amount).await;
    }
    async fn decrease(&mut self, amount: i32) {
        for handler in self.event_handler.iter() {
            handler.lock().await.on_decreased(amount).await;
        }

        // self.on_decreased(amount).await;
    }
}
#[derive(Clone)]
pub struct Client {
    intents: Intents,
    event_handler: EventHandlerList,
    token: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            intents: Intents { intents: 0 },
            event_handler: Vec::new(),
            token: "".to_string(),
        }
    }
    pub async fn start(mut self) {
        let cert_file = fs::read("./certs/localhost.crt").unwrap();
        let cert = native_tls::Certificate::from_pem(&cert_file).unwrap();
        let tls_connector = native_tls::TlsConnector::builder()
            // .add_root_certificate(cert)
            .build()
            .unwrap();
        let connector = tokio_tungstenite::Connector::NativeTls(tls_connector);
        let url = Url::parse("wss://gateway.discord.gg?v=10&encoding=utf8").unwrap();
        let (mut ws_stream, res) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, true, Some(connector))
                .await
                .expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();
        println!("{:?}", res);

        tokio::spawn(async move {
            println!("Ready to print incoming data");
            while let Some(message) = read.next().await {
                match message.unwrap() {
                    Message::Text(msg) => {
                        let data: Value =
                            serde_json::from_str(&msg).expect("There was a error processing event");
                        println!("{msg}");
                    }
                    _ => {}
                }
            }
        });
        tokio::spawn(async move {
            let data = json!({
                "op": 2,
                "d": {
                    "token": self.token,
                    "properties": {
                        "os": "linux",
                        "browser": "disco",
                        "device": "disco"
                    },
                    "compress": false,
                    "intents": self.intents.intents
                }
            });
            let _ = write.send(Message::Text(data.to_string())).await;
            loop {
                let _ = write
                    .send(Message::Text(
                        json!({
                            "op": 1,
                            "d": null
                        })
                        .to_string(),
                    ))
                    .await;
                tokio::time::sleep(Duration::from_millis(30000)).await;
            }
        });

        // tokio::spawn(async move {
        loop {}
        // });
        // loop {
        //     self.square("1".to_string()).await;
        //     self.increase(1).await;
        //     self.decrease(1).await;
        // }
    }
}

pub struct ClientBuilder {
    intents: Intents,
    event_handler: EventHandlerList,
    token: String,
}
impl ClientBuilder {
    pub fn add_event_handler(
        mut self,
        event_handler: impl EventHandler + Send + Sync + 'static,
    ) -> ClientBuilder {
        // intents::IntentsV2::GUILDS;
        self.event_handler
            .push(Arc::from(Mutex::new(event_handler)));
        self
    }
    pub fn set_intents(mut self, intents: Intents) -> ClientBuilder {
        self.intents = intents;
        self
    }
    pub fn set_token(mut self, token: impl AsRef<str>) -> ClientBuilder {
        self.token = token.as_ref().to_string();
        self
    }
    pub fn build(self) -> Client {
        Client {
            intents: self.intents,
            event_handler: self.event_handler,
            token: self.token,
        }
    }
}
