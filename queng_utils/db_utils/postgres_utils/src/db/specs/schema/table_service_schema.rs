use crate::db::all_db_constants::{SERVICE_TABLE, SYSTEM_SCHEMA};
use crate::db::Specs;
// Composite Types
//  https://www.postgresql.org/docs/current/rowtypes.html#ROWTYPES

// PostgreSQL CREATE TABLE
// https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-create-table/

// PostgreSQL CREATE INDEX
// https://www.postgresqltutorial.com/postgresql-indexes/postgresql-create-index/

impl Specs {
    pub(crate) fn generate_service_table_types_ddl(&self) -> String {
        r#"
        CREATE TYPE "metric_config" AS (
            uri  	 VARCHAR,
            host	 VARCHAR,
            port     smallint
        );

        CREATE TYPE "endpoint" AS (
            name     VARCHAR,
            version  smallint,
            base_uri VARCHAR,
            port     smallint,
            protocol smallint
        );
"#
        .to_string()
    }
    pub(crate) fn generate_service_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {SYSTEM_SCHEMA}.{SERVICE_TABLE} (
            id               smallint PRIMARY KEY,
            name             VARCHAR  UNIQUE NOT NULL,
            version          smallint NOT NULL,
            online           boolean NOT NULL,
            description      VARCHAR NOT NULL,
            health_check_uri VARCHAR NOT NULL,
            base_uri         VARCHAR NOT NULL,
            dependencies     smallint[],
            exposure         smallint NOT NULL,
            endpoint public.endpoint NOT NULL,
	        metrics public.metric_config NOT NULL
            );

            CREATE INDEX idx_service_id ON {SYSTEM_SCHEMA}.{SERVICE_TABLE}(id);
            CREATE INDEX idx_service_name ON {SYSTEM_SCHEMA}.{SERVICE_TABLE}(name);
            "
        )
    }
}
