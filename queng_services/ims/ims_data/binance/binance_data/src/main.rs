mod service;

use common_config::ServiceID;
use config_manager::CfgManager;

use mimalloc::MiMalloc;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let svc_config = ims_data_binance_specs::ims_data_binance_config();

    let _cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    Ok(())
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[IMS Data Binance/main]: {msg}");
    }
}
