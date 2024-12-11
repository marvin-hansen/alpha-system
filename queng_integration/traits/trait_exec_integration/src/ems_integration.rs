use common_errors::MessageProcessingError;
use common_order::{Balances, Positions};

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EmsIntegration: Send)]
pub trait LocalEmsIntegration {
    async fn get_balances(&self) -> Result<Balances, MessageProcessingError>;
    async fn get_positions(&self) -> Result<Positions, MessageProcessingError>;
}
