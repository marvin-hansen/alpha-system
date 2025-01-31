/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_database::ClickHouseConfig;

#[must_use]
pub fn get_ci_db_config() -> ClickHouseConfig {
    base_ci_db_config("metadata")
}

fn base_ci_db_config(db: &str) -> ClickHouseConfig {
    ClickHouseConfig::new(
        "127.0.0.1".to_string(),
        9000,
        "default".to_string(),
        String::new(),
        db.to_string(),
    )
}
