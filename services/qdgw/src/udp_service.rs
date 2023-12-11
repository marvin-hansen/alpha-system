use std::io;
use std::net::SocketAddr;

use tokio::net::UdpSocket;

pub struct UdpServer {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl UdpServer {
    pub fn new(socket: UdpSocket, buf: Vec<u8>, to_send: Option<(usize, SocketAddr)>) -> Self {
        Self { socket, buf, to_send }
    }
}

impl UdpServer {
    pub async fn run(self) -> Result<(), io::Error> {
        let UdpServer {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let data = &mut buf[..size];

                process_data(&data).await.expect("Failed to process data");

                let amt = socket.send_to(&mut buf[..size], &peer)
                    .await
                    .expect("Failed to send data ");

                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }

}

async fn process_data(data: &[u8]) -> Result<(), io::Error> {

    println!("Echoed {} ", String::from_utf8_lossy(data));

    Ok(())
}