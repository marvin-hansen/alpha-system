use crate::db::metadata::{Metadata, DB_NAME};

impl Metadata {
    pub fn generate_create_asset_table_ddl(&self) -> String {
        format!(
            "
    CREATE TABLE IF NOT EXISTS {DB_NAME}.assets
    (
        `code` String CODEC(LZ4),
        `name` String CODEC(LZ4),
        `asset_class` LowCardinality(String) CODEC(LZ4),
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
    SETTINGS index_granularity = 1024
    "
        )
    }

    pub fn generate_drop_asset_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {DB_NAME}.assets")
    }
}
