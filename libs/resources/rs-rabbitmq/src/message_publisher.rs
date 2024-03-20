use crate::ConnectionManager;
use amqprs::channel::BasicPublishArguments;
use amqprs::BasicProperties;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait MessagePublisher {
    async fn publish(&self, message: &str, routing_key: &str) -> Result<(), Box<dyn Error>>;
}

pub struct RabbitMqPublisher<T>
where
    T: ConnectionManager + Send + Sync,
{
    connection_manager: T,
    exchange: String,
}

impl<T> RabbitMqPublisher<T>
where
    T: ConnectionManager + Send + Sync,
{
    #[allow(dead_code)]
    pub fn new(connection_manager: T, exchange: String) -> Self {
        Self {
            connection_manager,
            exchange,
        }
    }
}

#[async_trait]
impl<T> MessagePublisher for RabbitMqPublisher<T>
where
    T: ConnectionManager + Send + Sync,
{
    async fn publish(&self, message: &str, routing_key: &str) -> Result<(), Box<dyn Error>> {
        let channel = self.connection_manager.create_channel().await?;

        // Prepare the BasicProperties
        let properties = BasicProperties::default();

        // Prepare the BasicPublishArguments
        let args = BasicPublishArguments {
            exchange: self.exchange.clone(),
            routing_key: routing_key.to_string(),
            mandatory: false,
            immediate: false,
        };

        channel
            .basic_publish(properties, message.as_bytes().to_vec(), args)
            .await?;

        Ok(())
    }
}
