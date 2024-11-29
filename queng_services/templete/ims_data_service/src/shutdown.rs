use common_iggy::IggyConfig;
use iggy::clients::client::IggyClient;
use std::error::Error;

/// Shuts down the Iggy producer and cleans up Iggy streams and topics.
///
/// # Errors
///
/// This function returns an error if the Iggy producer cannot be shut down.
///
/// # Panics
///
/// This function will panic if the Iggy stream and topic cannot be cleaned up.
///
pub(super) async fn shutdown(iggy_client: &IggyClient) -> Result<(), Box<dyn Error>> {
    println!("Shutting down");
    // Shutdown producer
    message_shared::shutdown(iggy_client)
        .await
        .expect("Failed to shutdown iggy producer");

    Ok(())
}

/// Shuts down and cleans up the Iggy client, including streams, topics, and user sessions.
///
/// # Arguments
///
/// * `iggy_client` - The `IggyClient` used for communication.
/// * `iggy_config` - Configuration details for the Iggy client.
///
/// # Returns
///
/// A `Result` indicating success or failure during the shutdown and cleanup process.
///
/// # Errors
///
/// This function returns an error if any of the shutdown or cleanup steps fail.
///
/// # Panics
///
/// This function will panic if the cleanup of streams and topics or the user logout fails.
pub(super) async fn shutdown_and_cleanup(
    iggy_client: &IggyClient,
    iggy_config: &IggyConfig,
) -> Result<(), Box<dyn Error>> {
    println!("Shutting down and cleaning up");

    message_shared::cleanup(iggy_client, iggy_config)
        .await
        .expect("Failed to clean up iggy");

    message_shared::logout_user(iggy_client)
        .await
        .expect("Failed to logout user");

    // Shutdown
    message_shared::shutdown(iggy_client)
        .await
        .expect("Failed to shutdown iggy consumer");

    Ok(())
}
