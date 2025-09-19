use crate::streaming::config::KafkaConfig;
use crate::streaming::consumer::EventConsumer;
use crate::streaming::printer::MessagePrinter;
use crate::streaming::producer::EventProducer;

mod producer;
mod consumer;
mod error;
mod config;
mod printer;

pub struct EventStream {
    producer: EventProducer,
    consumer: EventConsumer,
}

impl EventStream {
    pub fn new() -> anyhow::Result<Self> {
        let config = KafkaConfig::default();
        Ok(Self {
            producer: EventProducer::new(&config)?,
            consumer: EventConsumer::new(&config, MessagePrinter::new())?
        })
    }

    pub async fn send_test(&self) {
        let key = time::UtcDateTime::now().unix_timestamp().to_string();
        match self.producer.send_event(&key, "hello streaming").await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
    }
}