use crate::integration_message_config::IntegrationMessageConfig;
use crate::prelude::ImsIntegrationType;
pub use common_exchange::prelude::ExchangeID;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationConfig {
    integration_id: String,
    integration_version: u16,
    ims_integration_type: ImsIntegrationType,
    online: bool,
    exchange_id: ExchangeID,
    integration_message_config: IntegrationMessageConfig,
}

impl IntegrationConfig {
    /// Creates a new `IntegrationConfig` with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `integration_id`: The unique identifier for this integration.
    /// * `integration_version`: The version of this integration.
    /// * `ims_integration_type`: The type of integration this is.
    /// * `online`: Whether this integration is currently online.
    /// * `exchange_id`: The exchange with which this integration is associated.
    /// * `integration_message_config`: The configuration for the messages sent by this integration.
    ///
    /// # Returns
    ///
    /// A new `IntegrationConfig` with the given parameters.
    pub fn new(
        integration_id: String,
        integration_version: u16,
        ims_integration_type: ImsIntegrationType,
        exchange_id: ExchangeID,
        integration_message_config: IntegrationMessageConfig,
    ) -> Self {
        Self {
            integration_id,
            integration_version,
            ims_integration_type,
            online: false,
            exchange_id,
            integration_message_config,
        }
    }
}

impl IntegrationConfig {
    /// Returns the unique identifier for this integration.
    ///
    /// # Returns
    ///
    /// The unique identifier associated with this configuration.
    pub fn integration_id(&self) -> &str {
        &self.integration_id
    }

    /// Returns the version of this integration.
    ///
    /// # Returns
    ///
    /// The version associated with this configuration.
    pub fn integration_version(&self) -> u16 {
        self.integration_version
    }

    /// Returns the type of integration represented by this configuration.
    ///
    /// # Returns
    ///
    /// The `ImsIntegrationType` associated with this configuration.
    pub fn ims_integration_type(&self) -> ImsIntegrationType {
        self.ims_integration_type
    }

    /// Returns whether this integration is currently online.
    ///
    /// # Returns
    ///
    /// `true` if the integration is online, `false` otherwise.
    pub fn online(&self) -> bool {
        self.online
    }

    /// Returns the `ExchangeID` associated with this configuration.
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    /// Returns a reference to the `IntegrationMessageConfig` associated with this configuration.
    pub fn integration_message_config(&self) -> &IntegrationMessageConfig {
        &self.integration_message_config
    }
}

impl Display for IntegrationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.integration_id)
    }
}
