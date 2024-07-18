pub(crate) fn generate_drop_table_ddl(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {table_name}")
}
