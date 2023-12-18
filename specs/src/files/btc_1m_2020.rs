use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};

pub fn get_btc_usd_2020_file_config() -> FileConfig {
    FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "data/pqt/btcusd/btc_1m_year/btcusd_1m_2020.parquet".to_string(),
        "2020 BTC/USD 1 min data".to_string(),
        FileConfigType::BtcMin2020,
    )
}
