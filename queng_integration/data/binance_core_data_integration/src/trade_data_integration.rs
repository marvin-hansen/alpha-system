use crate::ImsBinanceDataIntegration;
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
            let stream_name = format!("{}@trade", symbol);

            let ws_stream = self.connect_websocket(&stream_name).await?;
            let processor = Arc::clone(&processor);

            let handle = tokio::spawn(async move {
                use futures_util::StreamExt;
                use tokio_tungstenite::tungstenite::Message;

                let (_, mut read) = ws_stream.split();

                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        // {
                        //   "e": "trade",       // Event type
                        //   "E": 1672515782136, // Event time
                        //   "s": "BNBBTC",      // Symbol
                        //   "t": 12345,         // Trade ID
                        //   "p": "0.001",       // Price
                        //   "q": "100",         // Quantity
                        //   "T": 1672515782136, // Trade time
                        //   "m": true,          // Is the buyer the market maker?
                        //   "M": true           // Ignore
                        // }
                        let data = text.as_bytes().to_vec();

                        if let Err(e) = processor.process(&[data]).await {
                            eprintln!("Error processing trade data: {}", e);
                            break;
                        }
                    }
                }
            });

            handlers.insert(symbol, handle);
        }

        Ok(())
    }

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
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
        }
        Ok(())
    }
}
