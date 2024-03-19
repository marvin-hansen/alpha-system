use common::prelude::DataType;
use std::future::Future;
use std::pin::Pin;

pub(crate) mod trade_handle;

// Returning an async function from a function:
// https://stackoverflow.com/questions/61167939/return-an-async-function-from-a-function-in-rust

pub(crate) fn get_stream_handler(
    data_type: DataType,
    symbols: Vec<String>,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    //
    return match data_type {
        DataType::UnknownDataType => Box::pin(trade_handle::market_websocket(symbols)),
        DataType::TradeData => Box::pin(trade_handle::market_websocket(symbols)),
        DataType::OHLCVData => Box::pin(trade_handle::market_websocket(symbols)),
    };
}
