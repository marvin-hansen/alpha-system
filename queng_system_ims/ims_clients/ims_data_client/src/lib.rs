mod handler;
mod shtudown;
mod types;

use common_ims::IntegrationConfig;
use common_ims::{IggyConfig, IggyUser};
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
pub use types::error::ImsDataClientError;

pub struct ImsDataClient {
    dbg: bool,
    client_id: u16,
    control_client: IggyClient,
    control_consumer: MessageConsumer,
    control_producer: MessageProducer,
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

        dbg_print("Create Identifiers for control stream and topic");
        let control_stream_id = integration_config.control_channel();
        let control_topic_id = integration_config.control_channel();

        dbg_print("Create Iggy user and config");
        let user = IggyUser::default();
        let iggy_config = IggyConfig::from_client_id(user, client_id);
        let control_client =
            message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
                .expect("Failed to build client");

        control_client.connect().await.expect("Failed to connect");

        control_client
            .login_user(iggy_config.user().username(), iggy_config.user().password())
            .await
            .expect("Failed to login user");

        dbg_print("Create control MessageProducer");
        let control_producer = MessageProducer::from_client(
            dbg,
            &control_client,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
        .await
        .expect("Failed to create producer");

        dbg_print("Create control MessageConsumer");
        let control_consumer = MessageConsumer::from_client(
            &control_client,
            "control_consumer",
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
        .await
        .expect("[Service]: Failed to create consumer");

        Ok(Self {
            dbg,
            client_id,
            control_client,
            control_consumer,
            control_producer,
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

    pub fn control_consumer(&self) -> &MessageConsumer {
        &self.control_consumer
    }

    pub fn control_producer(&self) -> &MessageProducer {
        &self.control_producer
    }

    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }

    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }
}

impl ImsDataClient {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ImsDataClient]: {msg}");
        }
    }
}
