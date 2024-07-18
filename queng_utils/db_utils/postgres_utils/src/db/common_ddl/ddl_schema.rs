pub(crate) fn generate_create_schema_ddl(schema_name: &str) -> String {
    format!("CREATE SCHEMA IF NOT EXISTS {schema_name};")
}

pub(crate) fn generate_drop_schema_ddl(schema_name: &str) -> String {
    format!("DROP SCHEMA IF EXISTS {schema_name} CASCADE;")
}
