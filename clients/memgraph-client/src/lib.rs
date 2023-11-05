use common::types::config::memgraph_config::MemGraphConfig;

pub struct MemGraphClient {
    config: MemGraphConfig,
    connection: rsmgclient::Connection,
}
