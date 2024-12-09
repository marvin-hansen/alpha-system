//! Example demonstrating the usage of Binance Coin Futures Data Integration
//!
//! This example shows how to:
//! 1. Create a Binance Coin Futures data integration instance
//! 2. Retrieve available symbols
//! 3. Validate symbols
//! 4. Start trade and OHLCV data streams
//! 5. Stop data streams

use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use common_errors::MessageProcessingError;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

/// A simple event processor that prints received data to the console.
/// In a real application, you might want to parse the JSON and process
/// the data more comprehensively.
#[derive(Debug)]
struct PrintEventProcessor;

impl EventProcessor for PrintEventProcessor {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), MessageProcessingError> {
        for msg in data {
            match String::from_utf8(msg.clone()) {
                Ok(text) => println!("Received data: {}", text),
                Err(e) => eprintln!("Error decoding message: {}", e),
            }
        }
        Ok(())
    }
}

/// Main example function demonstrating Binance Coin Futures data integration
#[tokio::main]
async fn main() -> Result<(), MessageProcessingError> {
    // Initialize rustls crypto provider for secure WebSocket connections
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    // Create Binance Coin Futures data integration instance
    let integration = ImsBinanceCoinFuturesDataIntegration::new();
    let processor = Arc::new(PrintEventProcessor);

    // Retrieve and print available symbols
    let symbols = integration.get_exchange_symbols().await?;
    println!("Available Coin Futures Symbols: {:?}", symbols);

    // Select a few symbols to stream (ensure they exist)
    let test_symbols = vec!["BNBUSD_PERP".to_string(), "ETHUSD_PERP".to_string()];

    // Validate the symbols
    integration.validate_symbols(&test_symbols).await?;

    // Start trade data stream
    println!("\nStarting trade data stream...");
    if let Err(e) = integration
        .start_trade_data(&test_symbols, Arc::clone(&processor))
        .await
    {
        eprintln!("✗ Failed to start trade data stream: {}", e);
        return Err(e);
    }
    println!("✓ Trade data stream started successfully!");

    // Start OHLCV data stream
    println!("\nStarting OHLCV data stream...");
    if let Err(e) = integration.start_ohlcv_data(&test_symbols, processor).await {
        eprintln!("✗ Failed to start OHLCV data stream: {}", e);
        // Make sure to stop trade stream if OHLCV stream fails
        integration.stop_all_trade_data().await?;
        return Err(e);
    }
    println!("✓ OHLCV data stream started successfully!");

    // Run for a limited time to demonstrate streaming
    time::sleep(Duration::from_secs(10)).await;

    // Stop trade data stream
    if let Err(e) = integration.stop_all_trade_data().await {
        eprintln!("✗ Error stopping trade data stream: {}", e);
    }

    // Stop OHLCV data stream
    if let Err(e) = integration.stop_all_ohlcv_data().await {
        eprintln!("✗ Error stopping OHLCV data stream: {}", e);
    }

    println!("✓ All streams stopped successfully!");

    Ok(())
}
