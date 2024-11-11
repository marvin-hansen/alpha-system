use crate::integration_message_config::IntegrationMessageConfig;
use crate::prelude::ImsIntegrationType;
pub use common_exchange::prelude::ExchangeID;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationConfig {
    integration_id: String,
    ims_integration_type: ImsIntegrationType,
    exchange_id: ExchangeID,
    integration_message_config: IntegrationMessageConfig,
}

impl IntegrationConfig {
    /// Creates a new `IntegrationConfig` with the given parameters.
    ///
    /// # Parameters
    ///
    /// - `integration_id`: The unique identifier for the integration.
    /// - `ims_integration_type`: The type of integration (e.g. OMS or Execution).
    /// - `exchange_id`: The identifier for the exchange associated with the integration.
    /// - `integration_message_config`: The configuration for the integration's message bus.
    ///
    /// # Returns
    ///
    /// A new `IntegrationConfig` with the given parameters.
    ///
    pub fn new(
        integration_id: String,
        ims_integration_type: ImsIntegrationType,
        exchange_id: ExchangeID,
        integration_message_config: IntegrationMessageConfig,
    ) -> Self {
        Self {
            integration_id,
            ims_integration_type,
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
    /// Returns the type of integration represented by this configuration.
    ///
    /// # Returns
    ///
    /// The `ImsIntegrationType` associated with this configuration.
    pub fn ims_integration_type(&self) -> ImsIntegrationType {
        self.ims_integration_type
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
