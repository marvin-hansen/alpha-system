use common_config::prelude::{ServiceConfig, ServiceID};
use common_exchange::prelude::Instrument as CommonInstrument;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
// use pg_mddb::prelude::Asset;

// pub fn get_test_asset() -> Asset {
//     Asset {
//         asset_code: "test_asset_code".to_string(),
//         asset_name: "test_asset_name".to_string(),
//         asset_classes: vec![],
//         asset_figi: None,
//     }
// }

pub fn get_test_instrument() -> CommonInstrument {
    CommonInstrument::new(
        "test_code".to_string(),
        "test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    )
}

pub fn get_test_portfolio() -> CommonPortfolioConfig {
    CommonPortfolioConfig::new(
        1,
        "Test Portfolio".to_string(),
        AccountType::Spot,
        "12345".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        30.0,
        10.0,
        500.0,
        1000.0,
        50.0,
        100.0,
    )
}

pub fn get_test_service_config() -> ServiceConfig {
    ServiceConfig::new(
        ServiceID::SMDB,
        "name".to_string(),
        1,
        true,
        "description".to_string(),
        "health_check_uri".to_string(),
        "base_uri".to_string(),
        vec![ServiceID::DBGW],
        Vec::from([
            common_config::prelude::Endpoint::default(),
            common_config::prelude::Endpoint::default(),
        ]),
    )
}
