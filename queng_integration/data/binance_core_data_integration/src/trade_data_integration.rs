use crate::ImsBinanceDataIntegration;
use common_data_bar::TradeBar;
use common_data_bar_ext::SbeTradeBarExtension;
use common_errors::MessageProcessingError;
use std::sync::Arc;
use trait_data_integration::{EventProcessor, ImsDataIntegration, ImsTradeDataIntegration};

impl ImsTradeDataIntegration for ImsBinanceDataIntegration {
    /// Starts real-time trade data streams for the specified symbols.
    ///
    /// This method:
    /// 1. Validates all symbols before establishing connections
    /// 2. Creates a WebSocket connection for each symbol
    /// 3. Spawns an async task to process incoming trade data
    /// 4. Stores task handles for lifecycle management
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `processor` - Event processor to handle incoming trade data
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are started successfully
    /// - `Err(MessageProcessingError)`: If symbol validation fails or connection errors occur
    ///
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        // Validate symbols first
        self.validate_symbols(symbols).await?;

        let mut handlers = self.trade_handlers.write().await;

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            if handlers.contains_key(&symbol) {
                // Symbol is already in the handlers collection, do nothing
                continue;
            }

            let stream_name = format!("{}@trade", symbol);
            let ws_stream = self.connect_websocket(&stream_name).await?;
            let processor = Arc::clone(&processor);

            let symbol_clone = symbol.clone();
            let handle = tokio::spawn(async move {
                use crate::utils;
                use futures_util::StreamExt;
                use tokio_tungstenite::tungstenite::Message;

                let (_, mut read) = ws_stream.split();
                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        let bar =
                            utils::extract_trade_bar_from_json(text.as_str(), &symbol_clone).await;

                        if let Some(bar) = bar {
                            let (_, data) =
                                TradeBar::encode_to_sbe(bar).expect("Failed to encode OHLCV data");
                            if let Err(e) = processor.process(&[data]).await {
                                eprintln!("Error processing OHLCV data: {}", e);
                                break;
                            }
                        }
                    }
                }
            });

            handlers.insert(symbol.clone(), handle);
            // Add symbol to active trade symbols list
            self.symbols_active_trade.write().await.push(symbol);
        }

        Ok(())
    }

    /// Stops real-time trade data streams for the specified symbols.
    ///
    /// This method:
    /// 1. Checks if the specified symbols are active trade streams
    /// 2. Aborts each handler immediately
    /// 3. Removes the handler from the storage
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If any symbols are not active trade streams
    ///
    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        // If no symbols provided, do nothing
        if symbols.is_empty() {
            return Ok(());
        }

        let mut handlers = self.trade_handlers.write().await;
        let mut stopped_symbols = Vec::new();
        let mut not_found_symbols = Vec::new();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            // If symbol is not in trade_handlers, track it
            if !handlers.contains_key(&symbol) {
                not_found_symbols.push(symbol.clone());
                continue;
            }

            // Remove and abort the handler for this symbol
            if let Some(handle) = handlers.remove(&symbol) {
                handle.abort();
                stopped_symbols.push(symbol.clone());
                // Remove symbol from active trade symbols list
                let mut active_symbols = self.symbols_active_trade.write().await;
                if let Some(pos) = active_symbols.iter().position(|s| s == &symbol) {
                    active_symbols.remove(pos);
                }
            }
        }

        // If any symbols were not found in trade_handlers, return an error
        if !not_found_symbols.is_empty() {
            return Err(MessageProcessingError::new(format!(
                "The following symbols were not active trade streams: {:?}",
                not_found_symbols
            )));
        }

        Ok(())
    }

    /// Stops all active trade data streams.
    ///
    /// This method:
    /// 1. Retrieves all active trade stream handlers
    /// 2. Aborts each handler immediately
    /// 3. Cleans up the handler storage
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If cleanup fails
    ///
    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        let mut handlers = self.trade_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
            // Clear active trade symbols list when stopping all streams
            self.symbols_active_trade.write().await.clear();
        }
        Ok(())
    }
}
