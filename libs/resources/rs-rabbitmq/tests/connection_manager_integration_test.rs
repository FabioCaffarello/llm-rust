use rs_rabbitmq::ConnectionManager;
use rs_rabbitmq::RabbitConnectionManager;

#[tokio::test]
async fn test_rabbitmq_connection() {
    let connection_manager = RabbitConnectionManager::new(
        String::from("localhost"), // Host
        5672,                      // Port
        String::from("guest"),     // Username
        String::from("guest"),     // Password
    );

    match connection_manager.get_connection().await {
        Ok(connection) => assert!(connection.is_open(), "Connection should be open"),
        Err(e) => panic!("Failed to connect to RabbitMQ: {:?}", e),
    }
}
