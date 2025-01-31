/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_errors::MessageProcessingError;
use common_order::{OrderCancel, OrderCancelAll, OrderCreate, OrderUpdate};

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EmsOrderIntegration: Send)]
pub trait LocalEmsOrderIntegration {
    /// Place an order
    async fn place_order(&self, order: &OrderCreate) -> Result<(), MessageProcessingError>;

    /// Update an existing order
    async fn update_order(&self, order: &OrderUpdate) -> Result<(), MessageProcessingError>;

    /// Cancel an existing order
    async fn cancel_order(&self, order: &OrderCancel) -> Result<(), MessageProcessingError>;

    /// Cancel all remaining orders
    async fn cancel_all_orders(&self, order: &OrderCancelAll)
        -> Result<(), MessageProcessingError>;
}
