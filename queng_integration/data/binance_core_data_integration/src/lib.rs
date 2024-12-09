mod ims_integration;
mod ohlcv_data_integration;
mod trade_data_integration;

use common_errors::MessageProcessingError;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

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
    api_base_url: String,
    api_wss_url: String,
    http_client: Client,
    symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>,
    trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
    ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}

impl ImsBinanceDataIntegration {
    pub fn new(api_base_url: &str, api_wss_url: &str) -> Self {
        Self {
            api_base_url: api_base_url.to_string(),
            api_wss_url: api_wss_url.to_string(),
            http_client: Client::new(),
            symbol_cache: RwLock::new(None),
            trade_handlers: RwLock::new(HashMap::new()),
            ohlcv_handlers: RwLock::new(HashMap::new()),
        }
    }
}

impl ImsBinanceDataIntegration {
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
        let url = format!("{}/{}", self.api_wss_url, stream_name);
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
