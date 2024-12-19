use binance_core_data_integration::ImsBinanceDataIntegration;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

// Binance USD-M Futures API endpoints
// https://binance-docs.github.io/apidocs/futures/en/#general-info
const API_BASE_URL: &str = "https://fapi.binance.com/fapi/v1";
const API_WSS_URL: &str = "wss://fstream.binance.com/ws";

pub struct ImsBinanceUsdFuturesDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceUsdFuturesDataIntegration {
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(API_BASE_URL, API_WSS_URL),
        }
    }
}

impl Default for ImsBinanceUsdFuturesDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsDataIntegration for ImsBinanceUsdFuturesDataIntegration {
    /// Retrieves the list of valid trading symbols for Binance USD-M Futures.
    ///
    /// # Returns
    /// A set of valid trading symbols or an error if retrieval fails.
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        self.integration.get_exchange_symbols().await
    }

    /// Validates the provided trading symbols against the list of valid Binance USD-M Futures symbols.
    ///
    /// # Arguments
    /// * `symbols` - A slice of trading symbols to validate
    ///
    /// # Returns
    /// `true` if all symbols are valid, or an error with details of invalid symbols.
    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}

impl ImsTradeDataIntegration for ImsBinanceUsdFuturesDataIntegration {
    /// Starts real-time trade data streams for the specified symbols in Binance USD-M Futures.
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols to start streaming (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `processor` - An event processor to handle incoming trade data
    ///
    /// # Returns
    /// `Ok(())` if streams are started successfully, or an error if setup fails.
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_trade_data(symbols, processor).await
    }

    /// Stops trade data streams for the specified symbols in Binance USD-M Futures.
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols to stop streaming
    ///
    /// # Returns
    /// `Ok(())` if streams are stopped successfully, or an error if any issues occur.
    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_trade_data(symbols).await
    }

    /// Stops all active trade data streams for Binance USD-M Futures.
    ///
    /// # Returns
    /// `Ok(())` if all streams are stopped successfully, or an error if any issues occur.
    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_trade_data().await
    }
}

impl ImsOhlcvDataIntegration for ImsBinanceUsdFuturesDataIntegration {
    /// Starts real-time OHLCV (candlestick) data streams for the specified symbols in Binance USD-M Futures.
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols to start streaming (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `processor` - An event processor to handle incoming OHLCV data
    ///
    /// # Returns
    /// `Ok(())` if streams are started successfully, or an error if setup fails.
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration
            .start_ohlcv_data(symbols, time_resolution, processor)
            .await
    }

    /// Stops OHLCV data streams for the specified symbols in Binance USD-M Futures.
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols to stop streaming
    ///
    /// # Returns
    /// `Ok(())` if streams are stopped successfully, or an error if any issues occur.
    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_ohlcv_data(symbols).await
    }

    /// Stops all active OHLCV data streams for Binance USD-M Futures.
    ///
    /// # Returns
    /// `Ok(())` if all streams are stopped successfully, or an error if any issues occur.
    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_ohlcv_data().await
    }
}
