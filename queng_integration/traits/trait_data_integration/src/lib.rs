use enum_dispatch::enum_dispatch;
use std::fmt::Error;

#[trait_variant::make(StreamProcessor: Send)]
pub trait LocalStreamProcessor {
    /// Callback to process a bar of trade data
    async fn process(&self, trades: &[Vec<u8>]) -> Result<(), Error>;
}

#[enum_dispatch(DataIntegration)]
#[trait_variant::make(ImsDataIntegration: Send)]
pub trait LocalImsDataIntegration {
    /// Start trade data for a list of symbols
    async fn start_trade_date<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: StreamProcessor + Send + Sync + 'static;

    /// Stop trade data for a list of symbols
    async fn stop_trade_date(&self, symbols: &[String]) -> Result<(), Error>;

    /// Stop all trade data
    async fn stop_all_trade_date(&self) -> Result<(), Error>;
}
