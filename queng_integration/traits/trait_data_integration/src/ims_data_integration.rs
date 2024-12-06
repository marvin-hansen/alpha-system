use common_errors::MessageProcessingError;
use std::sync::Arc;

/// Trait for a data integration that implements the `start_date`, `stop_date`, and `stop_all_date`
/// methods.
///
/// A data integration is used for fetching data from an exchange.
///
/// The `start_date` method is used to start fetching data from an exchange for the given symbols.
/// The method takes a `Vec<String>` of symbols to fetch data for and an `EventProcessor` that
/// will be called with the data.
///
/// The `stop_date` method is used to stop fetching data from an exchange for the given symbols.
///
/// The `stop_all_date` method is used to stop fetching data from all symbols.
///
#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsDataIntegration: Send)]
pub trait LocalImsDataIntegration {
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: crate::EventProcessor + Send + Sync + 'static;

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError>;

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: crate::EventProcessor + Send + Sync + 'static;

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>;
}
