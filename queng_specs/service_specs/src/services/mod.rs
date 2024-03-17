use common::prelude::ServiceConfig;

use crate::prelude::{cmdb_service_config, smdb_service_config};
use crate::services::dbgw::dbgw_service_config;
use crate::services::qdgw::qdgw_service_config;
use crate::services::symdb::symdb_service_config;
use crate::services::vex::vex_service_config;

pub mod cmdb;
pub mod dbgw;
pub mod qdgw;
pub mod smdb;
pub mod symdb;

pub(crate) mod ims_data;
pub mod vex;

pub fn get_all_service_configs() -> Vec<ServiceConfig> {
    vec![
        cmdb_service_config(),
        dbgw_service_config(),
        qdgw_service_config(),
        smdb_service_config(),
        symdb_service_config(),
        vex_service_config(),
    ]
}
