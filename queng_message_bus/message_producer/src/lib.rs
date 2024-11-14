use iggy::client_provider::ClientProviderConfig;

//
pub struct MessageProducer {
    client_config: ClientProviderConfig,
}

impl MessageProducer {
    pub fn client_config(&self) -> &ClientProviderConfig {
        &self.client_config
    }
}
