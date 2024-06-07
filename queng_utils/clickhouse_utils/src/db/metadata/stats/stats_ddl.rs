use crate::db::metadata::Metadata;

impl Metadata {
    pub fn generate_create_stats_table_ddl(&self) -> String {
        format!(
            "
    CREATE TABLE IF NOT EXISTS {DB_NAME}.assets
    (
        `download_timestamp` String CODEC(LZ4),
        `hash` String CODEC(LZ4),
        `number_assets` UInt32 CODEC(Delta, LZ4),
        `number_exchanges` UInt32 CODEC(Delta, LZ4),
        `number_instruments` UInt32 CODEC(Delta, LZ4),
    )
    ENGINE = MergeTree
    PRIMARY KEY (download_timestamp)
    SETTINGS index_granularity = 10
    "
        )
    }

    pub fn generate_drop_stats_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {DB_NAME}.stats")
    }
}
