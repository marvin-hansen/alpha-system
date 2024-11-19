use common_ims::IntegrationConfig;
use common_message::ImsDataConfig;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use std::collections::HashMap;
use std::error::Error;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
pub struct Server {
    dbg: bool,
    integration_config: IntegrationConfig,
    ims_data_config: ImsDataConfig,
    consumer: Guarded<MessageConsumer>,
    producer: MessageProducer,
    client_producers: Guarded<HashMap<u16, MessageProducer>>,
}

impl Server {
    /// Creates a new IMS data service server with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `ims_data_config` - Configuration for IMS data processing and authentication
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer
    /// * Failed to create the message producer
    /// * Failed to initialize communication channels
    pub async fn new(
        integration_config: IntegrationConfig,
        ims_data_config: ImsDataConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(false, integration_config, ims_data_config).await
    }

    /// Creates a new IMS data service server with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `ims_data_config` - Configuration for IMS data processing and authentication
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer
    /// * Failed to create the message producer
    /// * Failed to initialize communication channels
    pub async fn with_debug(
        integration_config: IntegrationConfig,
        ims_data_config: ImsDataConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(true, integration_config, ims_data_config).await
    }
}

impl Server {
    /// Builds a new server instance with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `dbg` - Whether to enable debug mode
    /// * `integration_config` - Configuration for integration endpoints and channels
    /// * `ims_data_config` - Configuration for IMS data processing and authentication
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Server` instance if successful, or a boxed error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Failed to create the message consumer with the specified credentials
    /// * Failed to create the message producer with the specified credentials
    /// * Failed to initialize the control or data channels
    /// * Failed to set up client producers
    async fn build(
        dbg: bool,
        integration_config: IntegrationConfig,
        ims_data_config: ImsDataConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let consumer_name = "ims-data-binance-control";
        let username = ims_data_config.stream_user().to_owned();
        let password = ims_data_config.stream_password().to_owned();
        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();
        let tcp_server_address = ims_data_config.tcp_server_address().to_owned();

        let consumer = MessageConsumer::new(
            consumer_name,
            username.clone(),
            password.clone(),
            stream_id,
            topic_id,
            tcp_server_address.clone(),
        )
        .await
        .expect("Failed to create consumer");

        let stream_id = integration_config.data_channel();
        let topic_id = integration_config.data_channel();
        let producer =
            MessageProducer::new(username, password, stream_id, topic_id, tcp_server_address)
                .await
                .expect("Failed to create producer");

        // Create a new HashMap to store data producers for each client
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        Ok(Self {
            dbg,
            integration_config,
            ims_data_config,
            consumer: std::sync::Arc::new(tokio::sync::RwLock::new(consumer)),
            producer,
            client_producers,
        })
    }

    /// Returns a reference to the thread-safe map of client producers.
    ///
    /// The client producers are stored in a thread-safe container using `Arc<RwLock>`,
    /// allowing for concurrent access from multiple tasks.
    ///
    /// # Returns
    ///
    /// A reference to the guarded HashMap containing client IDs mapped to their respective `MessageProducer`s.
    pub fn client_producers(&self) -> &Guarded<HashMap<u16, MessageProducer>> {
        &self.client_producers
    }

    /// Returns a reference to the thread-safe message consumer.
    ///
    /// The consumer is stored in a thread-safe container using `Arc<RwLock>`,
    /// allowing for concurrent access from multiple tasks.
    ///
    /// # Returns
    ///
    /// A reference to the guarded `MessageConsumer`.
    pub fn consumer(&self) -> &Guarded<MessageConsumer> {
        &self.consumer
    }

    /// Returns a reference to the message producer.
    ///
    /// # Returns
    ///
    /// A reference to the `MessageProducer` used for sending messages.
    pub fn producer(&self) -> &MessageProducer {
        &self.producer
    }
}
