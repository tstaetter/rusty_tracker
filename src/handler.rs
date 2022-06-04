//! Service handlers

use super::tracker_service::Event;

#[derive(Debug)]
pub struct MessageHandler {
    event: Event,
}

impl MessageHandler {
    pub fn new(event: Event) -> Self {
        MessageHandler { event }
    }

    pub async fn handle(&self) {
        println!("Got an event: {:?}", self.event);
    }
}
