use crate::common::all_db_constants::{SERVICE_TABLE, SERVICE_TABLE_INDEX, SYSTEM_SCHEMA};
use crate::db::Specs;

// PostgreSQL CREATE TABLE
// https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-create-table/

// PostgreSQL CREATE INDEX
// https://www.postgresqltutorial.com/postgresql-indexes/postgresql-create-index/

impl Specs {
    pub(crate) fn generate_service_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {SYSTEM_SCHEMA}.{SERVICE_TABLE} (
            id               INT4 PRIMARY KEY,
            name             VARCHAR  UNIQUE NOT NULL,
            version          INT2 NOT NULL,
            online           boolean NOT NULL,
            description      VARCHAR NOT NULL,
            health_check_uri VARCHAR NOT NULL,
            base_uri         VARCHAR NOT NULL,
            dependencies     INT2[],
            exposure         smallint NOT NULL,
            endpoint_name       VARCHAR,
            endpoint_version    INT2,
            endpoint_base_uri   VARCHAR,
            endpoint_port       INT2,
            endpoint_protocol  INT2,
            metric_uri  	    VARCHAR,
            metric_host	        VARCHAR,
            metric_port        INT2
            );
            "
        )
    }

    pub(crate) fn generate_service_table_index_ddl(&self) -> String {
        format!(
            "
            CREATE INDEX {SERVICE_TABLE_INDEX} ON {SYSTEM_SCHEMA}.{SERVICE_TABLE}(id);
            "
        )
    }
}
