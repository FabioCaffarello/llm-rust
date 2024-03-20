use rs_rabbitmq::{
    ConnectionManager, MessagePublisher, RabbitConnectionManager, RabbitMqPublisher,
};
use tokio::runtime::Runtime;

// A utility function to run async tests
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

#[test]
fn test_connection_manager_get_connection() {
    block_on(async {
        let manager = RabbitConnectionManager::new(
            "localhost".to_string(),
            5672,
            "guest".to_string(),
            "guest".to_string(),
        );
        // Now that ConnectionManager is in scope, get_connection should be available.
        let connection_result = manager.get_connection().await;
        assert!(connection_result.is_ok());
    });
}

#[test]
fn test_publisher_publish() {
    block_on(async {
        let manager = RabbitConnectionManager::new(
            "localhost".to_string(),
            5672,
            "guest".to_string(),
            "guest".to_string(),
        );
        let publisher = RabbitMqPublisher::new(manager, "test_exchange".to_string());
        // ConnectionManager is in scope, so create_channel and subsequently publish can be called.
        let publish_result = publisher
            .publish("Hello, RabbitMQ!", "test_routing_key")
            .await;
        assert!(publish_result.is_ok());
    });
}
