use async_trait::async_trait;
use std::sync::Arc;
use tokio;

use crate::intents;
use crate::intents::Intents;
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn on_squared(&self) {}
    async fn on_increased(&mut self, amount: i32) {}
    async fn on_decreased(&mut self, amount: i32) {}
}
impl EventHandler for Client {}
// pub trait EventEmitter {
//     fn square(&self) {}
//     fn increase(&mut self, amount: i32) {}
//     fn decrease(&mut self, amount: i32) {}
// }
// #[async_trait]
// impl EventEmitter for Client {
//     async fn square(&self) {
//         self.event_handler.iter().for_each(|handler| {
//             tokio::task::spawn(async move {});
//             handler.on_squared();
//         });
//         // Self::on_squared(self);
//     }
//     fn increase(&mut self, amount: i32) {
//         Self::on_increased(self, amount);
//     }
//     fn decrease(&mut self, amount: i32) {
//         Self::on_decreased(self, amount);
//     }
// }
pub struct Client {
    // intents: Vec<Intents>,
    event_handler: Vec<Arc<dyn EventHandler + Send + Sync + 'static>>,
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
    pub fn start(&mut self) {
        loop {
            Self::on_squared(self);
            Self::on_increased(self, 1);
            Self::on_decreased(self, 1);
        }
    }
}
pub struct ClientBuilder {
    // intents: Option<Vec<Intents>>,
    event_handler: Vec<Arc<dyn EventHandler + Send + Sync + 'static>>,
    token: Option<String>,
}
impl ClientBuilder {
    pub fn add_event_handler(
        mut self,
        event_handler: impl EventHandler + Send + Sync + 'static,
    ) -> ClientBuilder {
        // intents::IntentsV2::GUILDS;
        self.event_handler.push(Arc::from(event_handler));
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
