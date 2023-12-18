use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};

pub fn get_btc_usd_2017_file_config() -> FileConfig {
    FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "data/pqt/btcusd/btc_1m_year/btcusd_1m_2017.parquet".to_string(),
        "2017 BTC/USD 1 min data".to_string(),
        FileConfigType::BtcMin2017,
    )
}
