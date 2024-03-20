use crate::handlers::trade_handler::trade_event_handler;
use binance::websockets::{agg_trade_stream, WebSockets};
use binance::ws_model::WebsocketEvent;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) async fn trade_websocket(
    logger_tx: UnboundedSender<WebsocketEvent>,
    symbols: Vec<String>,
) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop

    let endpoints = get_endpoints(symbols);
    let mut web_socket: WebSockets<'_, WebsocketEvent> =
        WebSockets::new(|event: WebsocketEvent| {
            trade_event_handler(event, logger_tx.clone());
            Ok(())
        });

    web_socket.connect_multiple(endpoints).await.unwrap(); // check error
    println!("Connected to Binance websocket");

    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }

    web_socket.disconnect().await.unwrap();
    println!("Disconnected from Binance websocket");
}

fn get_endpoints(symbols: Vec<String>) -> Vec<String> {
    let mut endpoints: Vec<String> = Vec::with_capacity(symbols.len());

    for symbol in symbols {
        let endpoint = agg_trade_stream(&symbol);
        endpoints.push(endpoint);
    }

    endpoints
}
