use std::future::Future;
use std::io::Write;
use std::{error::Error, time::Duration};

use futures::FutureExt;
use tokio::{pin, select};
use zeromq::{Socket, SocketRecv, SocketSend};

pub struct Server {
    receiver: zeromq::PullSocket,
    sender: zeromq::PushSocket,
    controller: zeromq::SubSocket,
}

impl Server {
    pub fn new() -> Self {
        Self {
            receiver: zeromq::PullSocket::new(),
            sender: zeromq::PushSocket::new(),
            controller: zeromq::SubSocket::new(),
        }
    }
}

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), Box<dyn Error + Send>> {
        // If it is required to call .await on a &mut _ reference,
        // the caller is responsible for pinning the future.
        // https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        println!("Create Socket to receive messages ");
        let mut receiver = self.receiver;

        println!("Connect to Socket to receive messages ");
        let _ = receiver.connect("tcp://127.0.0.1:5557");

        println!("Create Socket to send messages");
        let mut sender = self.sender;

        println!("Connect to Socket to send messages ");
        let _ = sender.connect("tcp://127.0.0.1:5558");

        println!("Create Socket for control input");
        let mut controller = self.controller;

        let _ = controller.connect("tcp://127.0.0.1:5559");
        let _ = controller.subscribe("");

        println!("Starting main loop");
        loop {
            select! {

                _ = &mut signal_future => {
                    // println!("Closing all ZMQ connections");
                    sender.close().await;
                    receiver.close().await;
                    controller.close().await;
                    // println!("Exiting main loop");
                    break;
                }

                message = receiver.recv().fuse() => {
                    // Extract message
                    let message = message.unwrap();
                    let workload = String::from_utf8(message.get(0).unwrap().to_vec())
                    .expect("Failed to parse workload")
                        .parse()
                        .expect("Couldn't parse u64 from data");

                    // Do the work
                    process(Duration::from_millis(workload)).await;

                    // Send results to sink
                    let _ = sender.send(message).await;

                    // Simple progress indicator for the viewer
                    print!(".");
                    std::io::stdout().flush().expect("Failed to flush stdout");
            },

            }
        }

        Ok(())
    }
}

pub async fn process(duration: std::time::Duration) {
    tokio::time::sleep(duration).await
}
