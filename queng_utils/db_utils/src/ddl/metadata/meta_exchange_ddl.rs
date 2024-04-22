pub fn generate_create_exchanges_table_ddl() -> String {
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

pub fn generate_drop_exchanges_table_ddl() -> String {
    r"
    DROP TABLE IF EXISTS default.exchanges"
        .to_string()
}
