use all_data_integration::ImsDataIntegration;
use all_data_integration::ImsDataIntegration::MockDataIntegration as MockDataIntegrationEnum;
use mock_data_integration::MockDataIntegration;
use trait_data_integration::LocalImsDataIntegration;

#[tokio::test]
async fn test_all_data_integration() {
    let mock_data = ImsDataIntegration::from(MockDataIntegrationEnum(MockDataIntegration));

    // mock_data.start_trade_date(&["AAPL".to_string()], ).await.unwrap();
    mock_data
        .stop_trade_date(&["AAPL".to_string()])
        .await
        .unwrap();
    mock_data.stop_all_trade_date().await.unwrap();

    assert_eq!(1, 3)
}
