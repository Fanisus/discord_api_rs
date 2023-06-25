use crate::constants;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use reqwest::{self, Url};
use std::time::Duration;
use std::{fs, io::Write, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use native_tls;
type EventHandlerList = Vec<Arc<Mutex<dyn EventHandler + Send + Sync + 'static>>>;

#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn on_clientReady(&self, msg: String) {}
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
#[async_trait]
impl EventEmitter for Client {
    async fn square(&self, msg: String) {
        for handler in self.event_handler.iter() {
            handler.lock().await.on_clientReady(msg.clone()).await;
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
    // intents: Vec<Intents>,
    event_handler: EventHandlerList,
    token: String,
}
impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            // intents: None,
            event_handler: Vec::new(),
            token: None,
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
        let url = Url::parse("wss://gateway.discord.gg?v=9&encoding=utf8").unwrap();

        let (mut ws_stream, res) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, true, Some(connector))
                .await
                .expect("Failed to connect");
        println!("{:?}", res);
        let (mut write, mut read) = ws_stream.split();
        tokio::spawn(async move {
            println!("Ready to print incoming data");
            while let Some(message) = read.next().await {
                println!("{:?}", message);
            }
        });
        tokio::spawn(async move {
            let _ = write
                .send(Message::Text(
                    r#"{
            "op": 2,
            "d": {
              "token": "OTYzNDU1MjI5MDIwNDc1NDEy.G5f45k.uakG0G1B3q8hmUPHnKVLa5rGpk0fHQUrp6dAKk",
              "properties": {
                "os": "linux",
                "browser": "disco",
                "device": "disco"
              },
              "compress": false,
              "intents": 3276799
            }
          }"#
                    .to_string(),
                ))
                .await;
            loop {
                write
                    .send(Message::Text(
                        r#"{
                    "op": 1,
                    "d": null
                }"#
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
    // intents: Option<Vec<Intents>>,
    event_handler: EventHandlerList,
    token: Option<String>,
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
    // pub fn intents(mut self, intents: Vec<Intents>) -> ClientBuilder {}
    pub fn set_token(mut self, token: impl AsRef<str>) -> ClientBuilder {
        self.token = Some(token.as_ref().to_string());
        self
    }
    pub fn build(self) -> Client {
        Client {
            // intents: self.intents.expect("Expected Intents"),
            event_handler: self.event_handler,
            token: self.token.expect("Expected Token"),
        }
    }
}
