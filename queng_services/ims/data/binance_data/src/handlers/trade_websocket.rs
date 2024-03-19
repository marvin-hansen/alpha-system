use crate::handlers::trade_handler;
use binance::websockets::{agg_trade_stream, WebSockets};
use binance::ws_model::WebsocketEvent;
use std::sync::atomic::AtomicBool;

pub(crate) async fn trade_websocket(symbols: Vec<String>) {
    //
    let mut web_socket: WebSockets<'_, WebsocketEvent> =
        WebSockets::new(|event: WebsocketEvent| {
            trade_handler::trade_event_handler(event);
            Ok(())
        });

    let endpoints = get_endpoints(symbols);

    let mut secs = 1;
    let mut err_count = 0;

    // async client auto reconnect to server when disconnect
    // https://users.rust-lang.org/t/how-to-impl-async-client-auto-reconnect-to-server-when-disconnect/65587
    'reconnect_loop: loop {
        match web_socket.connect_multiple(endpoints.clone()).await {
            Ok(_) => {
                // Set or reset error count to 0 in case of a successful connection.
                err_count = 0;
                println!("Connected to stream");

                if let Err(e) = web_socket.event_loop(&AtomicBool::new(true)).await {
                    println!("Error starting event loop: {e}");
                }
            }

            // Implements exponential backoff for reconnecting to the stream.
            Err(e) => {
                err_count += 1;
                secs = secs * 2;
                tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;

                if err_count == 5 {
                    println!("Error: Failed to connect to stream: {}", e);
                    break 'reconnect_loop;
                }

                continue 'reconnect_loop;
            }
        }
    }

    web_socket
        .disconnect()
        .await
        .expect("Failed to disconnect from stream");
}

fn get_endpoints(symbols: Vec<String>) -> Vec<String> {
    let mut endpoints: Vec<String> = Vec::with_capacity(symbols.len());

    for symbol in symbols {
        let endpoint = agg_trade_stream(&symbol);
        endpoints.push(endpoint);
    }

    endpoints
}
