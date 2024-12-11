use common_errors::MessageProcessingError;
use common_order::{OrderCancelAll, OrderCancelSingle, OrderSingleNew, OrderSingleUpdate};

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EmsOrderIntegration: Send)]
pub trait LocalEmsOrderIntegration {
    /// Place an order
    async fn place_order(&self, order: &OrderSingleNew) -> Result<(), MessageProcessingError>;

    /// Update an existing order
    async fn update_order(&self, order: &OrderSingleUpdate) -> Result<(), MessageProcessingError>;

    /// Cancel an existing order
    async fn cancel_order(&self, order: &OrderCancelSingle) -> Result<(), MessageProcessingError>;

    /// Cancel all remaining orders
    async fn cancel_all_orders(&self, order: &OrderCancelAll)
        -> Result<(), MessageProcessingError>;
}
