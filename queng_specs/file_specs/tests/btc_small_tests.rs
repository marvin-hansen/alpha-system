use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};
use file_specs::prelude::get_btc_usd_small_file_config;

#[test]
fn test_get_btc_usd_small_file_config() {
    let expected = FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "/Users/marvin/RustroverProjects/quant-engine/data/pqt/btcusd/btcusd_1m_small.parquet".to_string(),
        "Small sample of BTC/USD 1 min data".to_string(),
        FileConfigType::BtcSmall,
    );

    let actual = get_btc_usd_small_file_config();

    assert_eq!(expected, actual);
}

