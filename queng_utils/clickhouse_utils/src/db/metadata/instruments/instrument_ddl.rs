use crate::db::metadata::{Metadata, TABLE_NAME};

impl Metadata {
    pub fn generate_create_instruments_table_ddl(&self) -> String {
        format!(
            "
     CREATE TABLE IF NOT EXISTS {TABLE_NAME}.instruments
     (
       `trade_start_timestamp` UInt64 CODEC(Delta, LZ4),
       `trade_end_timestamp` UInt64 CODEC(Delta, LZ4),
       `exchange_code` StringWithDictionary CODEC(LZ4),
       `exchange_pair_code` StringWithDictionary CODEC(LZ4),
       `base_asset` StringWithDictionary CODEC(LZ4),
       `quote_asset` StringWithDictionary CODEC(LZ4),
       `code` StringWithDictionary CODEC(LZ4),
       `class` StringWithDictionary CODEC(LZ4),
       `pair_figi` String CODEC(LZ4),
       `instrument_figi` String CODEC(LZ4),
     )
     ENGINE = MergeTree()
     PRIMARY KEY (code)
     ORDER BY (code, class, exchange_code, exchange_pair_code, base_asset, quote_asset)
     SETTINGS index_granularity = 2048
    "
        )
    }

    pub fn generate_drop_instruments_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {TABLE_NAME}.instruments")
    }
}
