use all_data_integration::{DataIntegration, LocalDataIntegrationTrait};
use common_ims::ExchangeDataIntegrationID::*;

#[tokio::test]
async fn test_all_data_integration_id() {
    let binance_data: DataIntegration = all_data_integration::get_data_integration(BinanceSpotData);
    let binance_usd_fut: DataIntegration =
        all_data_integration::get_data_integration(BinanceUsdFuturesData);
    let binance_coin_fut: DataIntegration =
        all_data_integration::get_data_integration(BinanceCoinFuturesData);

    let id = binance_data.id().await.unwrap();
    assert_eq!(id, "BinanceSpotData".to_string());

    let id = binance_usd_fut.id().await.unwrap();
    assert_eq!(id, "BinanceUsdFuturesData".to_string());

    let id = binance_coin_fut.id().await.unwrap();
    assert_eq!(id, "BinanceCoinFuturesData".to_string());

    assert_eq!(
        call_data_integration_id(binance_data).await,
        "BinanceSpotData".to_string()
    );

    assert_eq!(
        call_data_integration_id(binance_usd_fut).await,
        "BinanceUsdFuturesData".to_string()
    );

    assert_eq!(
        call_data_integration_id(binance_coin_fut).await,
        "BinanceCoinFuturesData".to_string()
    );
}

async fn call_data_integration_id(data_integration: DataIntegration) -> String {
    data_integration.id().await.unwrap()
}
