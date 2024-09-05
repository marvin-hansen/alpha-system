use common_metadata::prelude::Stats;
use pg_metadb::prelude::Stat;

#[test]
fn test_from_meta_stats() {
    let stats_download_timestamp = "test_timestamp".to_string();
    let meta_stats = Stats::new(stats_download_timestamp, "test_hash".to_string(), 10, 5, 20);
    let postgres_stat = Stat::from_meta_stats(meta_stats.clone());
    assert_eq!(postgres_stat.stats_hash, meta_stats.hash());
    assert_eq!(
        postgres_stat.stats_download_timestamp,
        meta_stats.download_timestamp()
    );
    assert_eq!(
        postgres_stat.stats_number_assets as u32,
        meta_stats.number_assets()
    );
    assert_eq!(
        postgres_stat.stats_number_exchanges as u32,
        meta_stats.number_exchanges()
    );
    assert_eq!(
        postgres_stat.stats_number_instruments as u32,
        meta_stats.number_instruments()
    );
}

#[test]
fn test_to_meta_stats() {
    let postgres_stat = Stat {
        stats_id: 1,
        stats_hash: "test_hash".to_string(),
        stats_download_timestamp: "test_timestamp".to_string(),
        stats_number_assets: 10,
        stats_number_exchanges: 5,
        stats_number_instruments: 20,
    };
    let meta_stats = postgres_stat.to_meta_stats();
    assert_eq!(meta_stats.hash(), &postgres_stat.stats_hash);
    assert_eq!(
        meta_stats.download_timestamp(),
        &postgres_stat.stats_download_timestamp
    );
    assert_eq!(
        meta_stats.number_assets(),
        postgres_stat.stats_number_assets as u32
    );
    assert_eq!(
        meta_stats.number_exchanges(),
        postgres_stat.stats_number_exchanges as u32
    );
    assert_eq!(
        meta_stats.number_instruments(),
        postgres_stat.stats_number_instruments as u32
    );
}
