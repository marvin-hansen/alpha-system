use crate::common::all_db_constants::{
    DEFAULT_SCHEMA, SERVICE_TABLE, SERVICE_TABLE_INDEX, SYSTEM_SCHEMA,
};
use crate::db::Specs;

// PostgreSQL CREATE TABLE
// https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-create-table/

// PostgreSQL CREATE INDEX
// https://www.postgresqltutorial.com/postgresql-indexes/postgresql-create-index/

// Check if a user-defined type already exists in PostgreSQL
// https://stackoverflow.com/questions/7624919/check-if-a-user-defined-type-already-exists-in-postgresql

impl Specs {
    pub(crate) fn generate_drop_protocol_type_ddl(&self) -> String {
        format!("DROP TYPE IF EXISTS {DEFAULT_SCHEMA}.ProtocolType CASCADE;")
    }

    pub(crate) fn generate_protocol_type_enum_ddl(&self) -> String {
        format!(
            "CREATE TYPE {DEFAULT_SCHEMA}.ProtocolType AS ENUM (
            'UnknownProtocol',
            'GRPC',
            'HTTP',
            'UDP'
            );"
        )
    }

    pub(crate) fn generate_drop_endpoint_type_ddl(&self) -> String {
        format!("DROP TYPE IF EXISTS {DEFAULT_SCHEMA}.Endpoint CASCADE;")
    }

    pub(crate) fn generate_endpoint_type_ddl(&self) -> String {
        format!(
            "CREATE TYPE {DEFAULT_SCHEMA}.Endpoint AS
            (
            name       VARCHAR,
            version    INT4,
            base_uri   VARCHAR,
            port       INT4,
            protocol   ProtocolType
            );"
        )
    }

    pub(crate) fn generate_service_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {SYSTEM_SCHEMA}.{SERVICE_TABLE}
            (
            id               INT4 PRIMARY KEY,
            name             VARCHAR  UNIQUE NOT NULL,
            version          INT2 NOT NULL,
            online           boolean NOT NULL,
            description      VARCHAR NOT NULL,
            health_check_uri VARCHAR NOT NULL,
            base_uri         VARCHAR NOT NULL,
            dependencies     INT2[],
            endpoints        Endpoint[] NOT NULL
            );"
        )
    }

    pub(crate) fn generate_service_table_index_ddl(&self) -> String {
        format!("CREATE INDEX {SERVICE_TABLE_INDEX} ON {SYSTEM_SCHEMA}.{SERVICE_TABLE}(id);")
    }
}
