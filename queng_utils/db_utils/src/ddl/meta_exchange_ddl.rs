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
    "
    .to_string()
}
