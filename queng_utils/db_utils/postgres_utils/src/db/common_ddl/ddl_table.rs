pub(crate) fn generate_drop_table_ddl(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {table_name}")
}

pub(crate) fn generate_verify_table_ddl(schema_name: &str, table_name: &str) -> String {
    format!(
        "SELECT EXISTS (
    SELECT FROM
        pg_tables
    WHERE
        schemaname = '{schema_name}' AND
        tablename  = '{table_name}'
    );"
    )
}
