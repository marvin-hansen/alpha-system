use client_utils::config_utils::ConfigFile;

pub(crate) fn get_trade_data_config() -> ConfigFile {
    ConfigFile::new("data/Kraken_Trading_History")
}
