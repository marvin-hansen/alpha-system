use std::{
    net::SocketAddr,
    time::Duration,
};

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use tarpc::context;
use tokio::time;

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
pub const PORT: u16 = 8080;

#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}

// This is the type that implements the generated World trait.
// It is the business logic and is used to start the server.
#[derive(Clone)]
pub struct HelloServer {
    socket_addr: SocketAddr,
}

impl HelloServer {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }
}

#[tarpc::server]
impl World for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        let sleep_time = Duration::from_millis(Uniform::new_inclusive(1, 10).sample(&mut thread_rng()));
        time::sleep(sleep_time).await;
        format!("Hello, {name}! You are connected from {}", self.socket_addr)
    }
}
