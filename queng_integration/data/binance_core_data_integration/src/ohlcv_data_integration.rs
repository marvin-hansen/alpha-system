use crate::utils;
use crate::ImsBinanceDataIntegration;
use common_data_bar::OHLCVBar;
use common_data_bar_ext::SbeOHLCVBarExtension;
use common_errors::MessageProcessingError;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;
use trait_data_integration::{EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration};

impl ImsOhlcvDataIntegration for ImsBinanceDataIntegration {
    /// Starts real-time OHLCV (candlestick) data streams for the specified symbols.
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        // Validate symbols first
        self.validate_symbols(symbols).await?;

        let mut handlers = self.ohlcv_handlers.write().await;

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            if handlers.contains_key(&symbol) {
                // Symbol is already in the handlers collection, do nothing
                continue;
            }

            let stream_name = format!("{}@kline_1m", symbol);

            let ws_stream = self.connect_websocket(&stream_name).await?;
            let processor = Arc::clone(&processor);

            let symbol_clone = symbol.clone();
            let handle = tokio::spawn(async move {
                let (_, mut read) = ws_stream.split();
                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        // Process the OHLCV data
                        let bar =
                            utils::extract_ohlcv_bar_from_json(text.as_str(), &symbol_clone).await;
                        if let Some(bar) = bar {
                            let (_, data) =
                                OHLCVBar::encode_to_sbe(bar).expect("Failed to encode OHLCV data");
                            if let Err(e) = processor.process(&[data]).await {
                                eprintln!("Error processing OHLCV data: {}", e);
                                break;
                            }
                        }
                    }
                }
            });

            handlers.insert(symbol, handle);
        }

        Ok(())
    }

    /// Stops real-time OHLCV data streams for the specified symbols.
    ///
    /// This method:
    /// 1. Checks if the specified symbols are active OHLCV streams
    /// 2. Aborts each handler immediately
    /// 3. Removes the handler from the storage
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If any symbols are not active OHLCV streams
    ///
    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        // If no symbols provided, do nothing
        if symbols.is_empty() {
            return Ok(());
        }

        let mut handlers = self.ohlcv_handlers.write().await;
        let mut stopped_symbols = Vec::new();
        let mut not_found_symbols = Vec::new();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            // If symbol is not in ohlcv_handlers, track it
            if !handlers.contains_key(&symbol) {
                not_found_symbols.push(symbol.clone());
                continue;
            }

            // Remove and abort the handler for this symbol
            if let Some(handle) = handlers.remove(&symbol) {
                handle.abort();
                stopped_symbols.push(symbol);
            }
        }

        // If any symbols were not found in ohlcv_handlers, return an error
        if !not_found_symbols.is_empty() {
            return Err(MessageProcessingError::new(format!(
                "The following symbols were not active OHLCV streams: {:?}",
                not_found_symbols
            )));
        }

        Ok(())
    }

    /// Stops all active OHLCV data streams.
    ///
    /// This method:
    /// 1. Retrieves all active OHLCV stream handlers
    /// 2. Aborts each handler immediately
    /// 3. Cleans up the handler storage
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If cleanup fails
    ///
    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        let mut handlers = self.ohlcv_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
        }
        Ok(())
    }
}
