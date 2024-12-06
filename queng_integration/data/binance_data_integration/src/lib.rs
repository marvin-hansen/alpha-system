use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug, Clone, Copy)]
pub struct ImsBinanceDataIntegration;

impl Default for ImsBinanceDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsBinanceDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImsDataIntegration for ImsBinanceDataIntegration {
    async fn start_trade_data<P>(&self, symbols: &[String], _processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("ImsBinanceDataIntegration start_trade_data");
        println!("Start data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_all_trade_data(&self) -> Result<(), Error> {
        println!("ImsBinanceDataIntegration stop_all_data");

        Ok(())
    }

    async fn start_ohlcv_data<P>(&self, symbols: &[String], _processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("ImsBinanceDataIntegration start_trade_data");
        println!("Start data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), Error> {
        println!("ImsBinanceDataIntegration stop_all_data");

        Ok(())
    }
}
