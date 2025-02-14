/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::Args;
use iggy::client_provider;
use iggy::client_provider::ClientProviderConfig;
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use std::sync::Arc;

/// Builds an Iggy client using the provided stream and topic identifiers.
///
/// # Arguments
///
/// * `stream_id` - The identifier of the stream.
/// * `topic_id` - The identifier of the topic.
///
/// # Returns
///
/// A `Result` wrapping the `IggyClient` instance or an `IggyError`.
///
pub async fn build_client(
    stream_id: String,
    topic_id: String,
    connect: bool,
) -> Result<IggyClient, IggyError> {
    let args = Args::new(stream_id, topic_id);

    build_client_from_args(args.to_sdk_args(), connect).await
}

/// Builds an Iggy client using the provided `Args`.
///
/// # Arguments
///
/// * `args` - The `Args` to use to build the client.
///
/// # Returns
///
/// A `Result` wrapping the `IggyClient` instance or an `IggyError`.
///
pub async fn build_client_from_args(
    args: iggy::args::Args,
    connect: bool,
) -> Result<IggyClient, IggyError> {
    // Build client provider configuration
    let client_provider_config = Arc::new(
        ClientProviderConfig::from_args(args).expect("Failed to create client provider config"),
    );

    // Build client_provider
    let client = match client_provider::get_raw_client(client_provider_config, connect).await {
        Ok(client) => client,
        Err(e) => return Err(IggyError::ClientNotFound(42)),
    };

    // Build client
    let client = match IggyClient::builder().with_client(client).build() {
        Ok(client) => client,
        Err(e) => return Err(e),
    };

    Ok(client)
}
