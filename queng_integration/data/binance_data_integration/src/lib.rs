use common_errors::MessageProcessingError;
use std::sync::Arc;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

pub struct ImsBinanceDataIntegration {}

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
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        Ok(())
    }

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        Ok(())
    }

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        Ok(())
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        Ok(())
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        Ok(true)
    }
}
