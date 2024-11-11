use common_exchange::prelude::ExchangeID;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationMessageConfig {
    id: u16,
    name: String,
    exchange_id: ExchangeID,
}

const NAME: &str = "integration";

impl IntegrationMessageConfig {
    pub fn new(id: u16, exchange_id: ExchangeID) -> Self {
        let name = format!("{}-{}-{}", exchange_id.to_string(), NAME, id);

        Self {
            id,
            name,
            exchange_id,
        }
    }
}
impl IntegrationMessageConfig {
    /// Returns the client id.
    ///
    /// # Returns
    ///
    /// A u16 representing the client id.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the name of the client.
    ///
    /// # Returns
    ///
    /// A string slice containing the name of the client.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ExchangeID of the client.
    ///
    /// # Returns
    ///
    /// An ExchangeID representing the id of the exchange the client is connected to.
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    /// Generates a channel name for the control channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-control".
    ///
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.name, "control")
    }

    /// Generates a channel name for the data channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-data".
    ///
    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.name, "data")
    }

    /// Generates a channel name for the error channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-error".
    pub fn error_channel(&self) -> String {
        format!("{}-{}", self.name, "error")
    }

    /// Generates a channel name for the execution channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-execution".
    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.name, "execution")
    }

    /// Generates a channel name for the heartbeat channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-heartbeat".
    pub fn heartbeat_channel(&self) -> String {
        format!("{}-{}", self.name, "heartbeat")
    }
}
