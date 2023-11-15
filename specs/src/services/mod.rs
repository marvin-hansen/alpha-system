use common::prelude::ServiceConfig;

use crate::prelude::{cmdb_service_config, memgraph_service_config, smdb_service_config};

pub mod cmdb;
pub mod dbgateway;
pub mod memgraph;
pub mod smdb;

pub fn get_all_service_configs() -> Vec<ServiceConfig> {
    vec![
        cmdb_service_config(),
        memgraph_service_config(),
        smdb_service_config(),
    ]
}
