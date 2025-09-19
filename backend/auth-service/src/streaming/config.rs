#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
    pub group_id: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string()),
            topic: std::env::var("KAFKA_TOPIC").unwrap_or("service.events.v1".to_string()),
            group_id: std::env::var("KAFKA_GROUP_ID").unwrap_or("service".to_string()),
            timeout_ms: std::env::var("KAFKA_TIMEOUT")
                .ok()
                .and_then(|timeout_ms| timeout_ms.parse::<u64>().ok())
                .unwrap_or(5000),
            max_retries: std::env::var("KAFKA_MAX_RETRIES")
                .ok()
                .and_then(|max_retries| max_retries.parse::<u32>().ok())
                .unwrap_or(5)
        }
    }
}