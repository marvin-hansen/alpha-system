mod api;
mod handler;
mod shutdown;
mod start;
mod types;

use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use common_ims::{IggyConfig, IggyUser};
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
pub use types::error::ImsDataClientError;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct ImsDataClient {
    dbg: bool,
    client_id: u16,
    control_client: IggyClient,
    data_client: IggyClient,
    data_consumer: Guarded<MessageConsumer>,
    data_handler: RwLock<Option<JoinHandle<()>>>,
    control_consumer: MessageConsumer,
    control_producer: MessageProducer,
    // control_handler: RwLock<JoinHandle<()>>,
    exchange_id: ExchangeID,
    iggy_config: IggyConfig,
    integration_config: IntegrationConfig,
}

impl ImsDataClient {
    /// Creates a new `ImsDataClient` instance with debugging disabled.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to use for this instance.
    /// * `integration_config` - The integration configuration for the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `ImsDataClient` instance or an error.
    ///
    pub async fn new(
        client_id: u16,
        integration_config: IntegrationConfig,
    ) -> Result<Self, ImsDataClientError> {
        Self::build(false, client_id, integration_config).await
    }

    /// Creates a new `ImsDataClient` instance with debugging enabled.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to use for this instance.
    /// * `integration_config` - The integration configuration for the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `ImsDataClient` instance or an error.
    ///
    pub async fn with_debug(
        client_id: u16,
        integration_config: IntegrationConfig,
    ) -> Result<Self, ImsDataClientError> {
        Self::build(true, client_id, integration_config).await
    }

    // Builds a new `ImsDataClient` instance.
    pub async fn build(
        dbg: bool,
        client_id: u16,
        integration_config: IntegrationConfig,
    ) -> Result<Self, ImsDataClientError> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[ImsDataClient]: {msg}");
            }
        };

        dbg_print("[ImsDataClient]: Get exchange id");
        let exchange_id = integration_config.exchange_id();

        dbg_print("[ImsDataClient]: Create Identifiers for control stream and topic");
        let control_stream_id = integration_config.control_channel();
        let control_topic_id = integration_config.control_channel();
        let data_stream_id = integration_config.data_channel();
        let data_topic_id = integration_config.data_channel();

        dbg_print("[ImsDataClient]: Create control channel, config, and client");
        let user = IggyUser::default();
        let iggy_config = IggyConfig::from_client_id(user, client_id);
        let control_client =
            message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
                .expect("[ImsDataClient]: Failed to build client");

        control_client
            .connect()
            .await
            .expect("[ImsDataClient]: Failed to connect to iggy bus on control topic");

        control_client
            .login_user(iggy_config.user().username(), iggy_config.user().password())
            .await
            .expect("[ImsDataClient]: Failed to login user");

        dbg_print("[ImsDataClient]: Create control MessageProducer");
        let control_producer = MessageProducer::from_client(
            dbg,
            &control_client,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
        .await
        .expect("[ImsDataClient]: Failed to create producer");

        dbg_print("[ImsDataClient]: Create control MessageConsumer");
        let control_consumer = MessageConsumer::from_client(
            dbg,
            &control_client,
            "control_consumer",
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
        .await
        .expect("[ImsDataClient]:  Failed to create consumer");

        dbg_print("[ImsDataClient]: Create data channel, config, and client");
        let user = IggyUser::default();
        let iggy_config = IggyConfig::from_client_id(user, client_id);
        let data_client =
            message_shared::build_client(data_stream_id.clone(), data_topic_id.clone())
                .await
                .expect("[ImsDataClient]: Failed to build client for data channel");

        dbg_print("[ImsDataClient]: Create data MessageConsumer");
        let data_consumer = MessageConsumer::from_client(
            dbg,
            &data_client,
            "data_consumer",
            data_stream_id.clone(),
            data_topic_id.clone(),
        )
        .await
        .expect("[ImsDataClient]: Failed to create consumer");

        let data_consumer = std::sync::Arc::new(RwLock::new(data_consumer));

        let data_handler = RwLock::new(None);

        Ok(Self {
            dbg,
            client_id,
            control_client,
            data_client,
            data_consumer,
            data_handler,
            control_consumer,
            control_producer,
            exchange_id,
            iggy_config,
            integration_config,
        })
    }
}

// Getters
impl ImsDataClient {
    pub fn dbg(&self) -> bool {
        self.dbg
    }

    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn control_client(&self) -> &IggyClient {
        &self.control_client
    }

    pub fn data_client(&self) -> &IggyClient {
        &self.data_client
    }

    pub fn control_consumer(&self) -> &MessageConsumer {
        &self.control_consumer
    }

    pub fn control_producer(&self) -> &MessageProducer {
        &self.control_producer
    }

    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }

    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }

    pub fn data_consumer(&self) -> &Guarded<MessageConsumer> {
        &self.data_consumer
    }

    pub fn data_handler(&self) -> &RwLock<Option<JoinHandle<()>>> {
        &self.data_handler
    }
}

impl ImsDataClient {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ImsDataClient]: {msg}");
        }
    }
}
