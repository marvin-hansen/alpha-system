use all_data_integration::{DataIntegration, LocalDataIntegrationTrait};
use common_ims::ExchangeDataIntegrationID::*;
use std::fmt::Error;
use std::future::Future;
use trait_data_integration::EventProcessor;

#[tokio::test]
async fn test_mock_integration_start_trade_data() {
    let mock_data: DataIntegration = all_data_integration::get_data_integration(MockData);
    let symbols = ["BTCUSDT".to_string()];
    let res = call_data_integration_start_trade_data(mock_data, &symbols).await;
    dbg!(&res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_mock_integration_stop_all_trade_data() {
    let mock_data: DataIntegration = all_data_integration::get_data_integration(MockData);
    let res = call_data_integration_stop_all_trade_data(mock_data).await;
    dbg!(&res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_mock_integration_start_ohlcv_data() {
    let mock_data: DataIntegration = all_data_integration::get_data_integration(MockData);
    let symbols = ["BTCUSDT".to_string()];
    let res = call_data_integration_start_ohlcv_data(mock_data, &symbols).await;
    dbg!(&res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_mock_integration_stop_all_ohlcv_data() {
    let mock_data: DataIntegration = all_data_integration::get_data_integration(MockData);
    let res = call_data_integration_stop_all_ohlcv_data(mock_data).await;
    dbg!(&res);
    assert!(res.is_ok());
}

struct PrintEventProcessor;

impl PrintEventProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventProcessor for PrintEventProcessor {
    fn process(&self, data: &[Vec<u8>]) -> impl Future<Output = Result<(), Error>> + Send {
        println!("data len: {}", data.len());
        let data = data.first().unwrap();
        println!("data first value: {:?}", String::from_utf8(data.to_owned()));
        async { Ok(()) }
    }
}

async fn call_data_integration_start_trade_data(
    data_integration: DataIntegration,
    symbols: &[String],
) -> Result<(), Error> {
    let p = PrintEventProcessor::new();

    data_integration.start_trade_data(symbols, p).await
}

async fn call_data_integration_stop_all_trade_data(
    data_integration: DataIntegration,
) -> Result<(), Error> {
    data_integration.stop_all_trade_data().await
}

async fn call_data_integration_start_ohlcv_data(
    data_integration: DataIntegration,
    symbols: &[String],
) -> Result<(), Error> {
    let p = PrintEventProcessor::new();

    data_integration.start_ohlcv_data(symbols, p).await
}

async fn call_data_integration_stop_all_ohlcv_data(
    data_integration: DataIntegration,
) -> Result<(), Error> {
    data_integration.stop_all_trade_data().await
}
