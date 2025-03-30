/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod getters;
mod utils;

use common_database::PostgresDBConfig;
use common_env::EnvironmentType;
use db_specs_postgres as postgres;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct PostgresConfigManager {
    dbg: bool,
    db_postgres_config: PostgresDBConfig,
}

impl PostgresConfigManager {
    #[must_use]
    pub fn new(env_type: &EnvironmentType) -> Self {
        Self::build(false, env_type)
    }

    #[must_use]
    pub fn with_debug(env_type: &EnvironmentType) -> Self {
        Self::build(true, env_type)
    }
    fn build(dbg: bool, env_type: &EnvironmentType) -> Self {
        Self {
            dbg,
            db_postgres_config: build_postgres_config(dbg, env_type),
        }
    }
}

fn build_postgres_config(dbg: bool, env_type: &EnvironmentType) -> PostgresDBConfig {
    if dbg {
        println!("[CfgManager]: get_postgres_config");
    }

    if env_type == &EnvironmentType::CLUSTER {
        // Env variables for the cluster are defined in:
        // delivery/infra/base
        let pg_user = utils::get_value_from_env("PG_USER");
        let pg_password = utils::get_value_from_env("PG_PASSWORD");
        let pg_database = utils::get_value_from_env("PG_DATABASE");

        // Get the cluster host
        let pg_host = postgres::get_cluster_db_host();
        postgres::get_cluster_db_config(pg_user, pg_password, pg_database, pg_host)
    } else {
        postgres::get_postgres_config(env_type)
    }
}

impl PostgresConfigManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresConfigManager]: {msg}");
        }
    }
}
