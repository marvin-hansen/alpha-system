use binance::ws_model::WebsocketEvent;
use common::prelude::DataType;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::UnboundedSender;

mod trade_handler;
pub(crate) mod trade_websocket;

// Returning an async function from a function:
// https://stackoverflow.com/questions/61167939/return-an-async-function-from-a-function-in-rust

pub(crate) fn _get_stream_handler(
    logger_tx: UnboundedSender<WebsocketEvent>,
    data_type: DataType,
    symbols: Vec<String>,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    //
    match data_type {
        DataType::UnknownDataType => Box::pin(trade_websocket::trade_websocket(logger_tx, symbols)),
        DataType::TradeData => Box::pin(trade_websocket::trade_websocket(logger_tx, symbols)),
        DataType::OHLCVData => Box::pin(trade_websocket::trade_websocket(logger_tx, symbols)),
    }
}
