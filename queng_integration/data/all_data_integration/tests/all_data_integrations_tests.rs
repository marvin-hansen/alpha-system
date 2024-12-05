use all_data_integration::{DataIntegration, LocalDataIntegrationTrait};
use common_ims::ExchangeDataIntegrationID::*;

#[tokio::test]
async fn test_all_data_integration_id() {
    let mock_data: DataIntegration = all_data_integration::get_data_integration(MockData);
    let binance_data: DataIntegration = all_data_integration::get_data_integration(BinanceData);
    let vex_data: DataIntegration = all_data_integration::get_data_integration(VexData);

    let id = mock_data.id().await.unwrap();
    assert_eq!(id, "MockDataIntegration".to_string());

    let id = binance_data.id().await.unwrap();
    assert_eq!(id, "BinanceDataIntegration".to_string());

    let id = vex_data.id().await.unwrap();
    assert_eq!(id, "VexDataIntegration".to_string());

    assert_eq!(
        call_data_integration_id(mock_data).await,
        "MockDataIntegration".to_string()
    );
    assert_eq!(
        call_data_integration_id(binance_data).await,
        "BinanceDataIntegration".to_string()
    );

    assert_eq!(
        call_data_integration_id(vex_data).await,
        "VexDataIntegration".to_string()
    );
}

async fn call_data_integration_id(data_integration: DataIntegration) -> String {
    data_integration.id().await.unwrap()
}
