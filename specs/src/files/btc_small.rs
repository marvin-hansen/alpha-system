use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};

pub fn get_btc_usd_small_file_config() -> FileConfig {
    FileConfig::new(
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "data/pqt/btcusd/btcusd_1m_small.parquet".to_string(),
        "Small sample of BTC/USD 1 min data".to_string(),
        FileConfigType::BtcSmall,
    )
}
