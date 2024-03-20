use amqprs::connection::{Connection, OpenConnectionArguments};
use async_trait::async_trait;
use std::error::Error;
use tokio::time::{sleep, Duration};

pub struct RabbitConnectionManager {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl RabbitConnectionManager {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self {
            host,
            port,
            username,
            password,
        }
    }
}

#[async_trait]
pub trait ConnectionManager {
    async fn get_connection(&self) -> Result<Connection, Box<dyn Error>>;
    async fn create_channel(&self) -> Result<amqprs::channel::Channel, Box<dyn Error>>;
}

#[async_trait]
impl ConnectionManager for RabbitConnectionManager {
    async fn get_connection(&self) -> Result<Connection, Box<dyn Error>> {
        let open_args =
            OpenConnectionArguments::new(&self.host, self.port, &self.username, &self.password)
                .virtual_host("/")
                .finish();

        let mut res = Connection::open(&open_args).await;

        while res.is_err() {
            println!("Attempting to reconnect after error...");
            sleep(Duration::from_secs(2)).await;
            res = Connection::open(&open_args).await;
        }

        res.map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn create_channel(&self) -> Result<amqprs::channel::Channel, Box<dyn Error>> {
        let connection = self.get_connection().await?;
        // Assuming `open_channel` requires an optional channel ID. If you don't have a specific ID, pass `None`.
        let channel = connection
            .open_channel(None)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(channel)
    }
}
