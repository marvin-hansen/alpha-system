use common_ims::{IggyConfig, IntegrationConfig};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_stream::MessageStream;
use std::collections::HashMap;
use std::error::Error;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
pub struct Service {
    dbg: bool,
    consumer: Guarded<MessageConsumer>,
    producer: Guarded<MessageProducer>,
    iggy_config: IggyConfig,
    integration_config: IntegrationConfig,
    client_configs: Guarded<HashMap<u16, IggyConfig>>,
    client_producers: Guarded<HashMap<u16, MessageStream>>,
}

impl Service {
    /// Creates a new server instance with debugging disabled.
    ///
    /// # Arguments
    ///
    /// * `consumer_client` - The Iggy client used for message consumption.
    /// * `producer_client` - The Iggy client used for message production.
    /// * `integration_config` - The integration configuration for the server.
    /// * `iggy_config` - The Iggy configuration for the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `Server` instance or an error.
    ///
    pub async fn new(
        consumer_client: &IggyClient,
        producer_client: &IggyClient,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(
            false,
            consumer_client,
            producer_client,
            integration_config,
            iggy_config,
        )
        .await
    }

    /// Creates a new server instance with debugging enabled.
    ///
    /// # Arguments
    ///
    /// * `consumer_client` - The Iggy client used for message consumption.
    /// * `producer_client` - The Iggy client used for message production.
    /// * `integration_config` - The integration configuration for the server.
    /// * `iggy_config` - The Iggy configuration for the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `Server` instance or an error.
    ///
    pub async fn with_debug(
        consumer_client: &IggyClient,
        producer_client: &IggyClient,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(
            true,
            consumer_client,
            producer_client,
            integration_config,
            iggy_config,
        )
        .await
    }
}

impl Service {
    async fn build(
        dbg: bool,
        consumer_client: &IggyClient,
        producer_client: &IggyClient,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[/Service]: {msg}");
            }
        };

        dbg_print("Create Identifiers for control stream and topic");
        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        dbg_print("Create MessageProducer");
        let producer =
            MessageProducer::from_client(producer_client, stream_id.clone(), topic_id.clone())
                .await
                .expect("Failed to create producer");
        let producer = std::sync::Arc::new(tokio::sync::RwLock::new(producer));

        dbg_print("Create MessageConsumer");
        let consumer = MessageConsumer::from_client(
            consumer_client,
            "control_consumer",
            stream_id.clone(),
            topic_id.clone(),
        )
        .await
        .expect("[Service]: Failed to create consumer");
        let consumer = std::sync::Arc::new(tokio::sync::RwLock::new(consumer));

        // Create a new HashMap to store data producers for each client
        dbg_print("Create HashMaps");
        let client_configs = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        dbg_print("Create Service");
        Ok(Self {
            dbg,
            consumer,
            producer,
            iggy_config: iggy_config.clone(),
            integration_config: integration_config.clone(),
            client_configs,
            client_producers,
        })
    }
}

impl Service {
    pub fn dbg(&self) -> bool {
        self.dbg
    }

    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }

    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }

    pub fn client_configs(&self) -> &Guarded<HashMap<u16, IggyConfig>> {
        &self.client_configs
    }

    pub fn client_producers(&self) -> &Guarded<HashMap<u16, MessageStream>> {
        &self.client_producers
    }
    pub fn consumer(&self) -> &Guarded<MessageConsumer> {
        &self.consumer
    }

    pub fn producer(&self) -> &Guarded<MessageProducer> {
        &self.producer
    }
}

impl Service {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}
