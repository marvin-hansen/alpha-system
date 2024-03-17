use common::prelude::ExchangeID;
use lib_data_stream;

const EXCHANGE_ID: ExchangeID = ExchangeID::Binance;

#[tokio::main]
async fn main() {
    lib_data_stream::run(EXCHANGE_ID)
        .await
        .expect("Failed to start data stream service for Binance ")
}
