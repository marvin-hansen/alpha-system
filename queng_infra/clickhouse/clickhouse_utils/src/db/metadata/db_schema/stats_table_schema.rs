/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::Metadata;
use crate::db::metadata::DB_NAME;
use crate::db::metadata::STATS_TABLE;
impl Metadata {
    pub(crate) fn generate_create_stats_table_ddl(&self) -> String {
        format!(
            "
    CREATE TABLE IF NOT EXISTS {DB_NAME}.{STATS_TABLE}
    (
        `download_timestamp` String CODEC(LZ4),
        `hash` String CODEC(LZ4),
        `number_assets` UInt32 CODEC(Delta, LZ4),
        `number_exchanges` UInt32 CODEC(Delta, LZ4),
        `number_instruments` UInt32 CODEC(Delta, LZ4),
    )
    ENGINE = MergeTree
    PRIMARY KEY (download_timestamp)
    SETTINGS index_granularity = 10
    "
        )
    }
}
