use crate::{BinaryProtocol, ConnectionClient, SwitchClient, SystemClient};

/// A client that can send and receive binary messages.
/// Implement the following traits to access all default methods:
/// * BinaryProtocol
/// * ConnectionClient
pub trait BinaryClient: Client + BinaryProtocol {}

pub trait Client: ConnectionClient + SystemClient + SwitchClient {}
