use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug, Clone, Copy)]
pub struct ImsBinanceDataIntegration;

impl ImsBinanceDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImsDataIntegration for ImsBinanceDataIntegration {
    async fn start_date<P>(&self, _symbols: &[String], _processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        Ok(())
    }

    async fn stop_date(&self, _symbols: &[String]) -> Result<(), Error> {
        Ok(())
    }

    async fn stop_all_date(&self) -> Result<(), Error> {
        Ok(())
    }
}
