pub(crate) fn generate_create_table_ddl(table_name: &str) -> String {
    format!("CREATE TABLE IF NOT EXISTS {table_name}")
}

pub(crate) fn generate_drop_table_ddl(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {table_name}")
}
