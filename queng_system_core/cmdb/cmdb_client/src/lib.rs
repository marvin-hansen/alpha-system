/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt;

use tonic::transport::{Channel, Uri};

use proto_cmdb::proto::cmdb_service_client::CmdbServiceClient;

mod prv_cmdb;

#[derive(Debug, Clone)]
pub struct CmdbClient {
    client: CmdbServiceClient<Channel>,
}

impl CmdbClient {
    pub async fn new(host: String, port: u16) -> Self {
        let s = format!("http://{host}:{port}");
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("[CMDBProvider]: Failed to parse server URI: {s}"));

        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n [CMDBProvider]: Failed to connect to SMDB service on: {s} \r\n  \r\n Detail: \r\n"));

        let client = CmdbServiceClient::new(channel);

        Self { client }
    }
}

#[derive(Debug)]
pub struct CMDBError(pub String);

impl Error for CMDBError {}

impl fmt::Display for CMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CMDBError: {}", self.0)
    }
}
