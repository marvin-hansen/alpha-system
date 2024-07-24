pub(crate) fn generate_count_table_query(schema_name: &str, table_name: &str) -> String {
    format!("SELECT COUNT(*) FROM {schema_name}.{table_name};")
}
