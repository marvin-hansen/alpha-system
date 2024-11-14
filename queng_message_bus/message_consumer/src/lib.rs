use iggy::client_provider::ClientProviderConfig;

//
pub struct MessageConsumer {
    client_config: ClientProviderConfig,
}

impl MessageConsumer {
    pub fn client_config(&self) -> &ClientProviderConfig {
        &self.client_config
    }
}
