use common::prelude::{FileConfig, FileConfigType, SymbolID, TimeResolution};

#[test]
fn test_new() {
    let config = FileConfig::new(
        1,
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "/path/to/file".to_string(),
        "BTC 1 min data".to_string(),
        FileConfigType::BtcMin2017,
    );

    assert_eq!(config.id(), 1);
    assert_eq!(config.data_symbol(), SymbolID::BTCUSD);
    assert_eq!(config.time_resolution(), TimeResolution::OneMin);
    assert_eq!(config.path(), "/path/to/file");
    assert_eq!(config.description(), "BTC 1 min data");
    assert_eq!(config.file_config_type(), &FileConfigType::BtcMin2017);
}

#[test]
fn test_display() {
    let config = FileConfig::new(
        1,
        SymbolID::BTCUSD,
        TimeResolution::OneMin,
        "/path/to/file".to_string(),
        "BTC 1 min data".to_string(),
        FileConfigType::BtcMin2017,
    );

    assert_eq!(
        format!("{}", config),
        "FileConfig[id=1, symbol=BTCUSD, resolution=OneMin, path=/path/to/file, description=BTC 1 min data, type=BtcMin2017]"
    );
}

