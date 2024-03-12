pub(crate) fn _generate_asset_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.assets
     (
       `timestamp` Datetime64(3) CODEC(DoubleDelta, LZ4),

     )
     ENGINE = MergeTree()
     PRIMARY KEY toStartOfHour(timestamp)
     SETTINGS index_granularity = 8192
    "
    .to_string()
}

pub(crate) fn _generate_exchange_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.exchanges
     (
       `timestamp` Datetime64(3) CODEC(DoubleDelta, ZSTD(1)),

     )
     ENGINE = MergeTree()
     PRIMARY KEY toStartOfHour(timestamp)
     SETTINGS index_granularity = 8192;
    "
    .to_string()
}

pub(crate) fn _generate_instruments_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.instruments
     (
       `timestamp` Datetime64(3) CODEC(DoubleDelta, ZSTD),

     )
     ENGINE = MergeTree()
     PRIMARY KEY toStartOfHour(timestamp)
     SETTINGS index_granularity = 8192;
    "
    .to_string()
}

pub(crate) fn _generate_master_symbols_table_ddl() -> String {
    r"
     CREATE TABLE IF NOT EXISTS default.master_symbols
     (
       `timestamp` Datetime64(3) CODEC(DoubleDelta, ZSTD(1)),
            symbol_id UInt64 CODEC(Delta, LZ4),
            symbol String,
     )
     ENGINE = MergeTree()
     PRIMARY KEY toStartOfHour(timestamp)
     SETTINGS index_granularity = 8192;
    "
    .to_string()
}
