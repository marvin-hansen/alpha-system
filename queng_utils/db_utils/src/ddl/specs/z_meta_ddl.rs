// remove after transition to formal meta-data model
pub fn generate_metadata_table_ddl(meta_data_table: &str) -> String {
    format!(
        r"
        CREATE TABLE IF NOT EXISTS default.{meta_data_table}
        (
            symbol String,
            symbol_id UInt64 CODEC(Delta, LZ4),
            table_name String,
            number_of_rows UInt64 CODEC(Delta, LZ4),
        )
        ENGINE = MergeTree
        PRIMARY KEY (symbol, symbol_id)
        SETTINGS index_granularity=128;
        "
    )
}
