use crate::handlers::get_stream_handler;
use crate::types::command::Command;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

pub async fn stream_manager(mut rx: mpsc::Receiver<Command>) {
    //
    let mut streams: HashMap<u32, UnboundedSender<bool>> = HashMap::new();

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::StartData(id, symbols, data_type_id) => {
                let (close_tx, mut close_rx) = mpsc::unbounded_channel::<bool>();
                let wait_loop = tokio::spawn(async move {
                    'wait_loop: loop {
                        select! {
                            _ = close_rx.recv() => break 'wait_loop
                        }
                    }
                });

                let stream_handler = get_stream_handler(data_type_id, symbols);
                tokio::spawn(stream_handler);
                streams.insert(id, close_tx.clone());

                select! {
                    _ = wait_loop => {}
                    _ = tokio::signal::ctrl_c() => {
                        println!("Closing websocket stream...");
                        close_tx.send(true).unwrap();
                    }
                }
            }
            Command::StopData(id) => {
                if let Some(close_tx) = streams.get(&id) {
                    close_tx.send(true).unwrap();
                    streams.remove(&id);
                }
            }
            Command::StopAllData => {
                // drain() clears the map, returning all key-value pairs as an iterator.
                // Keeps the allocated memory for reuse.
                for (_, close_tx) in streams.drain() {
                    close_tx.send(true).expect("Failed to send close signal");
                }
            }
        }
    }
}
