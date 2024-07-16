pub(crate) fn generate_create_db_ddl(db_name: &str) -> String {
    format!("CREATE DATABASE {db_name} WITH OWNER postgres TABLESPACE pg_default;")
}

pub(crate) fn generate_verify_db_ddl(db_name: &str) -> String {
    format!("SELECT datname FROM pg_catalog.pg_database WHERE datname = '{db_name}';")
}

pub(crate) fn generate_drop_db_ddl(db_name: &str) -> String {
    format!("DROP DATABASE IF EXISTS {db_name};")
}

pub(crate) fn generate_drop_table_ddl(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {table_name}")
}
