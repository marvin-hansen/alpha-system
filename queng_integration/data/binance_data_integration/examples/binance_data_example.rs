use binance_data_integration::ImsBinanceDataIntegration;
use common_errors::MessageProcessingError;
use std::sync::Arc;
use tokio::time::Duration;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug)]
struct PrintEventProcessor;

impl EventProcessor for PrintEventProcessor {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), MessageProcessingError> {
        for msg in data {
            if let Ok(text) = String::from_utf8(msg.clone()) {
                println!("Received data: {}", text);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    let integration = ImsBinanceDataIntegration::new();
    let processor = Arc::new(PrintEventProcessor);

    // Example symbols
    let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];

    // Validate symbols
    println!("Validating symbols...");
    integration.validate_symbols(&symbols).await?;
    println!("Symbols validated successfully!");

    // Start trade data stream
    println!("Starting trade data stream...");
    integration
        .start_trade_data(&symbols, Arc::clone(&processor))
        .await?;

    // Start OHLCV data stream
    println!("Starting OHLCV data stream...");
    integration.start_ohlcv_data(&symbols, processor).await?;

    // Let it run for 10 seconds
    println!("Streams started. Running for 10 seconds...");
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Stop all streams
    println!("Stopping all streams...");
    integration.stop_all_trade_data().await?;
    integration.stop_all_ohlcv_data().await?;
    println!("All streams stopped!");

    Ok(())
}
