use all_data_integration::{
    BinanceDataIntegration, DataIntegration, MockDataIntegration, TraitDataIntegration,
};

#[tokio::test]
async fn test_all_data_integration() {
    let mock_data: DataIntegration = MockDataIntegration.into();
    let binance_data: DataIntegration = BinanceDataIntegration.into();

    assert_eq!(mock_data.run().await.unwrap(), 20);
    assert_eq!(binance_data.run().await.unwrap(), 42);
}
