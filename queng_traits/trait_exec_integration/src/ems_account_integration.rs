/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_errors::MessageProcessingError;

/// The EMS account integration trait
/// Provides real-time updates for
/// * Account updates
/// * Position updates
/// * Order updates
#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EmsAccountIntegration: Send)]
pub trait LocalEmsAccountIntegration {
    /// Starts the account stream
    async fn start_account_stream(&self) -> Result<(), MessageProcessingError>;

    /// Stops the account stream
    async fn stop_account_stream(&self) -> Result<(), MessageProcessingError>;
}
