use crate::db::metadata::{Metadata, TABLE_NAME};

impl Metadata {
    pub fn generate_create_exchanges_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {TABLE_NAME}.exchanges
     (
       `code` String CODEC(LZ4),
       `name`String CODEC(LZ4),
       `active` Bool CODEC(LZ4),
       `url` String CODEC(LZ4),
         PROJECTION projection_exchanges_by_code
        (
            SELECT *
            GROUP BY
                code,
                name,
                active,
                url
        )
     )
    ENGINE = MergeTree
    PRIMARY KEY (code, name)
    SETTINGS index_granularity = 1024
    "
        )
    }

    pub fn generate_drop_exchanges_table_ddl(&self) -> String {
        r"
    DROP TABLE IF EXISTS default.exchanges"
            .to_string()
    }
}
