use common::prelude::{DataType, ExchangeID, HostEndpoint};
use ims_data_client::ImsDataClient;

fn get_config() -> HostEndpoint<'static> {
    HostEndpoint::new("127.0.0.1", 4040)
}

//
// Requires that the IMS DATA server is running on localhost:4040
// Start the server with:
//
// cargo run --bin binance_data
//

#[tokio::test]
async fn test_start_stop_single_stream() {
    let config = get_config();

    let mut client = ImsDataClient::new(config).await.unwrap();
    let exchange_id = ExchangeID::Binance;
    let symbols = vec![String::from("btceth")];
    let data_type = DataType::TradeData;

    let result = client.start_data(exchange_id, symbols, data_type).await;
    assert!(result.is_ok());

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let stream_id = result.unwrap();

    let result = client.stop_data(exchange_id, stream_id, data_type).await;
    assert!(result.is_ok());
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
