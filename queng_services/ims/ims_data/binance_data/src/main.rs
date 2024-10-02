// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
mod handlers;
mod service;
mod stream_manager;

use common_config::prelude::ServiceID;
use config_manager::CfgManager;

use mimalloc::MiMalloc;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create auto configuration
    let svc_config = ims_data_binance_specs::ims_data_binance_config();

    let _cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    Ok(())
}
