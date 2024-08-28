use common_exchange::prelude::Instrument as CommonInstrument;

pub fn get_instrument() -> CommonInstrument {
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
