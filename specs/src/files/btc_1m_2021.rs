use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};

pub fn get_btc_usd_2021_file_config() -> FileConfig {
    FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "/Users/marvin/RustroverProjects/quant-engine/data/pqt/btcusd/btc_1m_year/btcusd_1m_2021.parquet".to_string(),
        "2021 BTC/USD 1 min data".to_string(),
        FileConfigType::BtcMin2021,
    )
}
