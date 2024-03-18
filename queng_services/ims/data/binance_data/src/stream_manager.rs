use crate::handlers::trade_handle::market_websocket;
use crate::types::command::Command;
use std::collections::HashMap;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

const ORDER: atomic::Ordering = atomic::Ordering::Relaxed;

pub async fn stream_manager(mut rx: mpsc::Receiver<Command>) {
    let counter = AtomicU64::new(0);

    let mut streams: HashMap<u16, UnboundedSender<bool>> = HashMap::new();

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::StartData(symbols, data_type_id) => {
                let id = counter.fetch_add(1, ORDER) as u16;
                let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
                let wait_loop = tokio::spawn(async move {
                    'wait_loop: loop {
                        select! {
                            _ = close_rx.recv() => break 'wait_loop
                        }
                    }
                });

                tokio::spawn(Box::pin(market_websocket()));
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
                }
            }
            Command::StopAllData => {
                for (id, close_tx) in streams.drain() {
                    close_tx.send(true).expect("Failed to send close signal");
                }
            }
            Command::ReconnectData(id) => {
                if let Some(close_tx) = streams.get(&id) {
                    close_tx.send(true).unwrap();
                }
                let (close_tx, mut close_rx) = mpsc::unbounded_channel::<bool>();
                let wait_loop = tokio::spawn(async move {
                    'wait_loop: loop {
                        select! {
                        _ = close_rx.recv() => break 'wait_loop
                        }
                    }
                });
            }
        }
    }
}
