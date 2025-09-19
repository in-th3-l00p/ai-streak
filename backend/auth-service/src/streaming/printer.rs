use async_trait::async_trait;
use crate::streaming::consumer::MessageHandler;
use crate::streaming::error::KafkaResult;

pub struct MessagePrinter {}

impl MessagePrinter {
    pub fn new() -> Box<Self> {
        Box::new(MessagePrinter {})
    }
}

#[async_trait]
impl MessageHandler for MessagePrinter {
    async fn handle(&self, key: &[u8], payload: &[u8]) -> KafkaResult<()> {
        tracing::info!(
            "received message {} -> {}",
            String::from_utf8_lossy(key),
            String::from_utf8_lossy(payload)
        );

        Ok(())
    }
}