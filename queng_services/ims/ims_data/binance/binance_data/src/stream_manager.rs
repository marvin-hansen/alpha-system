use crate::handlers::trade_websocket;
use binance::ws_model::WebsocketEvent;
use ims_common::BinanceDataCommand;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

pub async fn stream_manager(mut command_rx: mpsc::Receiver<BinanceDataCommand>) {
    //
    let mut streams: HashMap<u32, UnboundedSender<bool>> = HashMap::new();

    while let Some(cmd) = command_rx.recv().await {
        match cmd {
            BinanceDataCommand::Start(id, symbols, _data_type_id) => {
                let (logger_tx, mut logger_rx) = mpsc::unbounded_channel::<WebsocketEvent>();
                let (close_tx, mut close_rx) = mpsc::unbounded_channel::<bool>();

                let wait_loop = tokio::spawn(async move {
                    'wait_loop: loop {
                        select! {
                            event = logger_rx.recv() => {
                                println!("{event:?}")
                            },
                            _ = close_rx.recv() => break 'wait_loop
                        }
                    }
                });

                let stream_handler = Box::pin(trade_websocket::trade_websocket(logger_tx, symbols));
                tokio::spawn(stream_handler);
                streams.insert(id, close_tx.clone());

                select! {
                    _ = wait_loop => {}
                    _ = tokio::signal::ctrl_c() => {
                         println!("[StartData]: Closing websocket stream...");
                         match close_tx.send(true){
                                Ok(_) => {
                                    println!("[StartData]: Successfully closed websocket stream");
                                }
                                Err(_) => {
                                    println!("[StartData]: Failed to close websocket stream");
                                }
                            }
                    }
                }
            }
            BinanceDataCommand::Stop(id) => {
                if let Some(close_tx) = streams.get(&id) {
                    println!("[StopData]: Closing websocket stream: {}", id);
                    match close_tx.send(true) {
                        Ok(_) => {
                            println!("[StopData]: Successfully closed websocket stream: {}", id);
                        }
                        Err(_) => {
                            println!("[StopData]: Failed to close websocket stream: {}", id);
                        }
                    }

                    streams.remove(&id);
                }
            }
            BinanceDataCommand::StopAll => {
                // drain() clears the map, returning all key-value pairs as an iterator.
                // Keeps the allocated memory for reuse.
                for (id, close_tx) in streams.drain() {
                    println!("[StopAllData]: Closing websocket stream: {}", id);
                    match close_tx.send(true) {
                        Ok(_) => {
                            println!(
                                "[StopAllData]: Successfully closed websocket stream: {}",
                                id
                            );
                        }
                        Err(_) => {
                            println!("[StopAllData]: Failed to close websocket stream: {}", id);
                        }
                    }
                }
            }
        }
    }
}
