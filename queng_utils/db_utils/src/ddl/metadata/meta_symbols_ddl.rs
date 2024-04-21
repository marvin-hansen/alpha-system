pub fn generate_master_symbols_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.master_symbols
     (
            `master_symbol_id` UInt64 CODEC(Delta, LZ4),
            `master_symbol` String CODEC(LZ4),
            `asset_class` String CODEC(LZ4),
            `base_asset` String CODEC(LZ4),
            `quote_asset` String CODEC(LZ4),
     )
     ENGINE = MergeTree()
     PRIMARY KEY (master_symbol_id, master_symbol)
     ORDER BY (master_symbol_id, master_symbol)
     SETTINGS index_granularity = 2048
    "
    .to_string()
}
