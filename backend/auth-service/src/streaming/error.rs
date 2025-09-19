use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("failed to create Kafka client: {0}")]
    ClientCreation(String),

    #[error("failed to send message: {0}")]
    MessageSend(String),

    #[error("message delivery failed: {0}")]
    MessageDelivery(String),

    #[error("failed to deserialize message: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("connection timeout: {0}")]
    Timeout(String),
}

pub type KafkaResult<T> = Result<T, KafkaError>;