use common_errors::MessageProcessingError;
use reqwest::Client;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

const BINANCE_API_BASE: &str = "https://api.binance.com/api/v3";
const BINANCE_WS_BASE: &str = "wss://stream.binance.com:9443/ws";
const SYMBOL_CACHE_DURATION: Duration = Duration::from_secs(7200); // 120 minutes

pub struct ImsBinanceDataIntegration {
    http_client: Client,
    symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>,
    trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
    ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}

impl Default for ImsBinanceDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsBinanceDataIntegration {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            symbol_cache: RwLock::new(None),
            trade_handlers: RwLock::new(HashMap::new()),
            ohlcv_handlers: RwLock::new(HashMap::new()),
        }
    }

    async fn get_valid_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        // Check cache first
        if let Some((symbols, timestamp)) = &*self.symbol_cache.read().await {
            if timestamp.elapsed() < SYMBOL_CACHE_DURATION {
                return Ok(symbols.clone());
            }
        }

        // Cache is stale or doesn't exist, fetch from API
        let url = format!("{}/exchangeInfo", BINANCE_API_BASE);
        let response =
            self.http_client.get(&url).send().await.map_err(|e| {
                MessageProcessingError::new(format!("Failed to fetch symbols: {}", e))
            })?;

        let data: Value = response
            .json()
            .await
            .map_err(|e| MessageProcessingError::new(format!("Failed to parse response: {}", e)))?;

        let symbols = data["symbols"]
            .as_array()
            .ok_or_else(|| MessageProcessingError::new("Invalid response format".to_string()))?
            .iter()
            .filter_map(|s| s["symbol"].as_str().map(String::from))
            .collect::<HashSet<_>>();

        // Update cache
        *self.symbol_cache.write().await = Some((symbols.clone(), Instant::now()));

        Ok(symbols)
    }

    async fn connect_websocket(
        &self,
        stream_name: &str,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        MessageProcessingError,
    > {
        let url = format!("{}/{}", BINANCE_WS_BASE, stream_name);
        let (ws_stream, _) = tokio_tungstenite::connect_async_with_config(
            url,
            Some(WebSocketConfig::default()),
            true,
        )
        .await
        .map_err(|e| MessageProcessingError::new(format!("WebSocket connection failed: {}", e)))?;
        Ok(ws_stream)
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

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        let mut handlers = self.trade_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
        }
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
        // Validate symbols first
        self.validate_symbols(symbols).await?;

        let mut handlers = self.ohlcv_handlers.write().await;

        for symbol in symbols {
            let symbol = symbol.to_lowercase();
            let stream_name = format!("{}@kline_1m", symbol);

            let ws_stream = self.connect_websocket(&stream_name).await?;
            let processor = Arc::clone(&processor);

            let handle = tokio::spawn(async move {
                use futures_util::StreamExt;
                use tokio_tungstenite::tungstenite::Message;

                let (_, mut read) = ws_stream.split();

                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        let data = text.as_bytes().to_vec();
                        if let Err(e) = processor.process(&[data]).await {
                            eprintln!("Error processing OHLCV data: {}", e);
                            break;
                        }
                    }
                }
            });

            handlers.insert(symbol, handle);
        }

        Ok(())
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        let mut handlers = self.ohlcv_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
        }
        Ok(())
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        let valid_symbols = self.get_valid_symbols().await?;

        let invalid_symbols: Vec<_> = symbols
            .iter()
            .filter(|s| !valid_symbols.contains(*s))
            .collect();

        if !invalid_symbols.is_empty() {
            return Err(MessageProcessingError::new(format!(
                "The following symbols are not traded on Binance: {:?}",
                invalid_symbols
            )));
        }

        Ok(true)
    }
}
