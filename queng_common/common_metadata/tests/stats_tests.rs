/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_metadata::MetaStats;

#[test]
fn test_meta_stats_new_initialization() {
    let download_timestamp = "2023-10-01T12:00:00Z".to_string();
    let hash = "abc123".to_string();
    let number_assets = 100;
    let number_exchanges = 10;
    let number_instruments = 50;

    let meta_stats = MetaStats::new(
        download_timestamp.clone(),
        hash.clone(),
        number_assets,
        number_exchanges,
        number_instruments,
    );

    assert_eq!(meta_stats.download_timestamp(), download_timestamp);
    assert_eq!(meta_stats.hash(), hash);
    assert_eq!(meta_stats.number_assets(), number_assets);
    assert_eq!(meta_stats.number_exchanges(), number_exchanges);
    assert_eq!(meta_stats.number_instruments(), number_instruments);
}

#[test]
fn test_meta_stats_download_timestamp() {
    let download_timestamp = "2023-10-01T12:00:00Z".to_string();
    let meta_stats = MetaStats::new(
        download_timestamp.clone(),
        "abc123".to_string(),
        100,
        10,
        50,
    );

    assert_eq!(meta_stats.download_timestamp(), download_timestamp);
}

#[test]
fn test_meta_stats_hash() {
    let hash = "abc123".to_string();
    let meta_stats = MetaStats::new(
        "2023-10-01T12:00:00Z".to_string(),
        hash.clone(),
        100,
        10,
        50,
    );

    assert_eq!(meta_stats.hash(), hash);
}
