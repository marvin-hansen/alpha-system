use std::fmt::Error;

/// Local trait to define an `EventProcessor` that can be used to process events
/// in a local context.
///
/// The `process` method is a callback that is called with the data fetched from
/// the exchange. The method takes a `&[Vec<u8>]` of data as input and returns a
/// `Result` of `()`.
#[trait_variant::make(EventProcessor: Send)]
pub trait LocalEventProcessor {
    /// Callback to process data
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), Error>;
}

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
#[trait_variant::make(ImsDataIntegration: Send)]
pub trait LocalImsDataIntegration {
    async fn start_date<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_date(&self, symbols: &[String]) -> Result<(), Error>;

    async fn stop_all_date(&self) -> Result<(), Error>;
}
