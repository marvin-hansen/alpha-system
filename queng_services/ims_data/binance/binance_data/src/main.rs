use common_config::ServiceID;
use config_manager::CfgManager;
use mimalloc::MiMalloc;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::Default;
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    let svc_config = binance_data_specs::ims_data_binance_config();
    let integration_config = binance_data_specs::binance_ims_data_integration_config();
    let iggy_config = binance_data_specs::ims_data_iggy_config();
    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    ims_data_service::start(DBG, &integration_config, &iggy_config, cfg_manager)
        .await
        .expect("Failed to start Binance IMS Data service");

    Ok(())
}
