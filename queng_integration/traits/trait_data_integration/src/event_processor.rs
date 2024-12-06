use std::fmt::Error;

/// Trait to define an `EventProcessor` that can be used to process events
/// in a local context.
///
/// The `process` method is a callback that is called with the data fetched from
/// the exchange. The method takes a `&[Vec<u8>]` of data as input and returns a
/// `Result` of `()`.
#[trait_variant::make(EventProcessor: Send)]
pub trait LocalEventProcessor {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), Error>;
}
