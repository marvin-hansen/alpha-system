use crate::prelude::{
    cmdb_service_config, dbgw_service_config, ims_data_binance_config, qdgw_service_config,
    smdb_service_config, symdb_service_config, vex_service_config,
};
use common::prelude::{ServiceConfig, ServiceID};

pub mod prelude;
pub mod services;

pub fn get_service_config(svc: &ServiceID) -> ServiceConfig {
    match svc {
        ServiceID::Default => ServiceConfig::default(),
        ServiceID::CMDB => cmdb_service_config(),
        ServiceID::DBGW => dbgw_service_config(),
        ServiceID::QDGW => qdgw_service_config(),
        ServiceID::SMDB => smdb_service_config(),
        ServiceID::SYMDB => symdb_service_config(),
        ServiceID::VEX => vex_service_config(),
        ServiceID::ImsDataBinance => ims_data_binance_config(),
    }
}
