use binance::websockets::{agg_trade_stream, WebSockets};
use binance::ws_model::WebsocketEvent;
use std::sync::atomic::AtomicBool;

// #[allow(dead_code)]
pub(crate) async fn market_websocket() {
    let agg_trade: String = agg_trade_stream("ethbtc");

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

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&AtomicBool::new(true)).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}
