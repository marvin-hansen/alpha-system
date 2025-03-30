/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod error;
mod mddb_assets;
mod mddb_exchanges;
mod mddb_instruments;

use proto_mddb::proto::mddb_service_client::MddbServiceClient;
use std::fmt::Error;
use tonic::transport::{Channel, Uri};

/// Client for interacting with the MDDB.
///
/// Wraps a `MddbServiceClient` and provides methods to
/// lookup symbols, symbol IDs, and exchange names.
///
#[derive(Debug, Clone)]
pub struct MDDBClient {
    client: MddbServiceClient<Channel>,
}

impl MDDBClient {
    /// Creates a new `SymdbClient` instance.
    ///
    /// # Arguments
    ///
    /// * `config: HostEndpoint` - The endpoint configuration of the SYMDB Service gRPC server
    ///
    /// # Returns
    ///
    /// Returns a `SymdbClient` connected to the given address.
    ///
    pub async fn new(host: String, port: u16) -> Result<Self, Error> {
        // "http://[::1]:7070"
        let s = format!("http://{host}:{port}");

        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("\r\n ❌ [MDDBClient]: Failed to parse server URI: {s}"));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[MDDBClient]: Failed to connect to MDDB service on: {s} \r\n  \r\n Detail: \r\n"));

        let client = MddbServiceClient::new(channel);

        Ok(Self { client })
    }
}
