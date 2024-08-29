use common_exchange::prelude::Instrument as CommonInstrument;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};

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