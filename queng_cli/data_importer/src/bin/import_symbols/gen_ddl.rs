pub fn _generate_asset_table_ddl() -> String {
    r"
    CREATE TABLE IF NOT EXISTS default.assets
    (
        `code` String CODEC(LZ4),
        `name` String CODEC(LZ4),
        `asset_class` StringWithDictionary CODEC(LZ4),
        `asset_figi` String CODEC(LZ4),

        PROJECTION projection_assets_by_class
        (
            SELECT *
            GROUP BY
                code,
                name,
                asset_class,
                asset_figi
        )
    )
    ENGINE = MergeTree
    PRIMARY KEY (code, name, asset_figi)
    SETTINGS index_granularity = 192
    "
    .to_string()
}

pub fn generate_exchange_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.exchanges
     (
       `code` String CODEC(LZ4),
       `name`String CODEC(LZ4),
       `active` Bool CODEC(LZ4),
       `url` String CODEC(LZ4),
     )
     ENGINE = MergeTree()
     PRIMARY KEY (code, name)
     SETTINGS index_granularity = 12;
    "
    .to_string()
}

pub fn _generate_instruments_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.instruments
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
     SETTINGS index_granularity = 812;
    "
    .to_string()
}

pub fn _generate_master_symbols_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.master_symbols
     (
            `master_symbol_id` UInt64 CODEC(Delta, LZ4),
            `master_symbol` String CODEC(LZ4),
     )
     ENGINE = MergeTree()
     PRIMARY KEY (master_symbol_id, master_symbol)
     ORDER BY (master_symbol_id, master_symbol)
     SETTINGS index_granularity = 8192;
    "
    .to_string()
}
