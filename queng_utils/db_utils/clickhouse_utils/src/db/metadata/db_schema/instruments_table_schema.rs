use crate::db::metadata::Metadata;
use crate::db::metadata::DB_NAME;
use crate::db::metadata::INSTRUMENTS_TABLE;
impl Metadata {
    pub(crate) fn generate_create_instruments_table_ddl(&self) -> String {
        format!(
            "
     CREATE TABLE IF NOT EXISTS {DB_NAME}.{INSTRUMENTS_TABLE}
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
            PROJECTION projection_instruments_by_code
            (
                SELECT *
                GROUP BY
                trade_start_timestamp,
                trade_end_timestamp,
                    exchange_code,
                    exchange_pair_code,
                    base_asset,
                    quote_asset,
                    code,
                    class,
                    pair_figi,
                    instrument_figi
            )
     )
     ENGINE = MergeTree
     PRIMARY KEY (code, pair_figi)
     SETTINGS index_granularity = 2048
     ;
    "
        )
    }
}
