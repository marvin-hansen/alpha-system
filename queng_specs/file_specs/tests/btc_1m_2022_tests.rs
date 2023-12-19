use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};
use file_specs::prelude::get_btc_usd_2022_file_config;

#[test]
fn test_get_btc_usd_2022_file_config() {
    let expected = FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "/Users/marvin/RustroverProjects/quant-engine/data/pqt/btcusd/btc_1m_year/btcusd_1m_2022.parquet".to_string(),
        "2022 BTC/USD 1 min data".to_string(),
        FileConfigType::BtcMin2022,
    );

    let actual = get_btc_usd_2022_file_config();

    assert_eq!(expected, actual);
}
