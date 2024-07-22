use crate::common::all_db_constants::DEFAULT_SCHEMA;

// https://www.postgresql.org/docs/current/sql-droptype.html

pub(crate) fn generate_drop_type_ddl(type_name: &str) -> String {
    format!("DROP TYPE IF EXISTS {DEFAULT_SCHEMA}.{type_name} CASCADE;")
}
