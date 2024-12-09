//! Example of using Binance Testnet Data Integration
//!
//! This example demonstrates how to:
//! 1. Create a Binance Testnet data integration instance
//! 2. Validate trading symbols
//! 3. Start trade data streams
//! 4. Process incoming trade data
//! 5. Stop trade data streams

use binance_testnet_data_integration::ImsBinanceTestnetDataIntegration;
use common_errors::MessageProcessingError;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use trait_data_integration::{EventProcessor, ImsDataIntegration, ImsTradeDataIntegration};

/// A simple event processor that prints received trade data to the console.
/// In a real application, you might want to parse the JSON and process
/// the data according to your needs.
#[derive(Debug)]
struct TradeDataProcessor {
    symbol: String,
}

impl EventProcessor for TradeDataProcessor {
    async fn process(&self, events: &[Vec<u8>]) -> Result<(), MessageProcessingError> {
        for event in events {
            let event_str = String::from_utf8_lossy(event);
            println!("Received trade data for {}: {}", self.symbol, event_str);
        }
        Ok(())
    }
}

/// Main example function demonstrating Binance Testnet data integration
#[tokio::main]
async fn main() -> Result<(), MessageProcessingError> {
    // Initialize rustls crypto provider for secure WebSocket connections
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    // Create Binance Testnet data integration instance
    let integration = ImsBinanceTestnetDataIntegration::new();

    // Retrieve and print exchange symbols
    let symbols = integration.get_exchange_symbols().await?;
    println!("Available Testnet Symbols: {:?}", symbols);

    // Select a few symbols to stream (ensure they exist in the testnet)
    let test_symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];

    // Validate the symbols
    integration.validate_symbols(&test_symbols).await?;

    // Create a processor for each symbol
    let processors: Vec<_> = test_symbols
        .iter()
        .map(|symbol| {
            Arc::new(TradeDataProcessor {
                symbol: symbol.clone(),
            })
        })
        .collect();

    // Start trade data streams
    for (symbol, processor) in test_symbols.iter().zip(processors.iter()) {
        integration
            .start_trade_data(&[symbol.clone()], Arc::clone(processor))
            .await?;
    }

    // Run for a limited time to demonstrate streaming
    time::sleep(Duration::from_secs(10)).await;

    // Stop trade data streams
    integration.stop_trade_data(&test_symbols).await?;

    Ok(())
}
