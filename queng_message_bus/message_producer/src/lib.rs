use iggy::client_provider::ClientProviderConfig;

//
pub struct MessageProducer {
    client_config: ClientProviderConfig,
    topic: String,
    message: String,
}
