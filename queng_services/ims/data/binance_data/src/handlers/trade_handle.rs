use binance::websockets::{agg_trade_stream, WebSockets};
use binance::ws_model::WebsocketEvent;
use std::sync::atomic::AtomicBool;
pub(crate) async fn market_websocket(symbols: Vec<String>) {
    let mut web_socket: WebSockets<'_, WebsocketEvent> =
        WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::Trade(trade) => {
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        trade.symbol, trade.price, trade.qty
                    );
                }
                _ => (),
            };

            Ok(())
        });

    let endpoints = get_endpoints(symbols);

    web_socket
        .connect_multiple(endpoints)
        .await
        .expect("Failed to connect to stream");
    if let Err(e) = web_socket.event_loop(&AtomicBool::new(true)).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
}

fn get_endpoints(symbols: Vec<String>) -> Vec<String> {
    let mut endpoints: Vec<String> = Vec::with_capacity(symbols.len());

    for symbol in symbols {
        let endpoint = agg_trade_stream(&symbol);
        endpoints.push(endpoint);
    }

    endpoints
}
