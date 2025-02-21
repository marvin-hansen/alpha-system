/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::DB_NAME;
use crate::db::metadata::EXCHANGES_TABLE;
use crate::db::metadata::Metadata;
impl Metadata {
    pub(crate) fn generate_create_exchanges_table_ddl(&self) -> String {
        format!(
            "
     CREATE TABLE IF NOT EXISTS {DB_NAME}.{EXCHANGES_TABLE}
     (
       `code` String CODEC(LZ4),
       `name` String CODEC(LZ4),

        PROJECTION projection_exchanges_by_code
        (
            SELECT *
            GROUP BY
                code,
                name
        )
     )
    ENGINE = MergeTree
    PRIMARY KEY (code, name)
    SETTINGS index_granularity = 1024
    "
        )
    }
}
