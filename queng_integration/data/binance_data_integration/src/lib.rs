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

/// A Binance data integration implementation that provides real-time trade and OHLCV data streams.
///
/// This struct implements the `ImsDataIntegration` trait for the Binance cryptocurrency exchange.
/// It manages WebSocket connections for trade and OHLCV data streams, handles symbol validation,
/// and provides efficient caching of exchange information.
///
/// # Features
/// - Real-time trade data streaming via WebSocket
/// - Real-time OHLCV (candlestick) data streaming
/// - Symbol validation with caching
/// - Thread-safe connection management
/// - Automatic cleanup of terminated connections
///
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
    /// Creates a new instance of `ImsBinanceDataIntegration`.
    ///
    /// This constructor initializes:
    /// - An HTTP client for REST API calls
    /// - A symbol cache for efficient symbol validation
    /// - Thread-safe storage for WebSocket connection handlers
    ///
    /// # Examples
    ///
    /// ```
    /// use binance_data_integration::ImsBinanceDataIntegration;
    ///
    /// let integration = ImsBinanceDataIntegration::new();
    /// ```
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            symbol_cache: RwLock::new(None),
            trade_handlers: RwLock::new(HashMap::new()),
            ohlcv_handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Retrieves and caches the list of valid trading symbols from Binance.
    ///
    /// This method:
    /// 1. Checks the cache first, returning cached symbols if they're still valid
    /// 2. If cache is stale or empty, fetches fresh symbols from Binance API
    /// 3. Updates the cache with new symbols and timestamp
    ///
    /// The cache duration is set to 120 minutes (2 hours) to balance API rate limits
    /// with symbol list accuracy.
    ///
    /// # Returns
    /// - `Ok(HashSet<String>)`: Set of valid trading symbols
    /// - `Err(MessageProcessingError)`: If API call fails or response is invalid
    ///
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

    /// Establishes a WebSocket connection to the Binance streaming API.
    ///
    /// # Arguments
    /// * `stream_name` - The name of the stream to connect to (e.g., "btcusdt@trade")
    ///
    /// # Returns
    /// - `Ok(WebSocketStream)`: Connected WebSocket stream
    /// - `Err(MessageProcessingError)`: If connection fails
    ///
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

    /// Starts real-time OHLCV (candlestick) data streams for the specified symbols.
    ///
    /// This method:
    /// 1. Validates all symbols before establishing connections
    /// 2. Creates a WebSocket connection for each symbol's 1-minute kline stream
    /// 3. Spawns an async task to process incoming OHLCV data
    /// 4. Stores task handles for lifecycle management
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `processor` - Event processor to handle incoming OHLCV data
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are started successfully
    /// - `Err(MessageProcessingError)`: If symbol validation fails or connection errors occur
    ///
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

    /// Validates a list of trading symbols against Binance's supported symbols.
    ///
    /// This method:
    /// 1. Retrieves the current list of valid symbols (using cache when possible)
    /// 2. Checks each input symbol against the valid symbol list
    /// 3. Returns an error if any symbols are invalid
    ///
    /// # Arguments
    /// * `symbols` - List of symbols to validate
    ///
    /// # Returns
    /// - `Ok(true)`: If all symbols are valid
    /// - `Err(MessageProcessingError)`: If any symbols are invalid, with error message listing invalid symbols
    ///
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
