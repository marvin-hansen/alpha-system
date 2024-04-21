pub fn generate_asset_table_ddl() -> String {
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
    SETTINGS index_granularity = 1024
    "
    .to_string()
}
