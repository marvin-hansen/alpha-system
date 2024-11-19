pub fn generate_drop_table_ddl(table_name: &str, db_dbname: &str) -> String {
    format!("DROP TABLE IF EXISTS {db_dbname}.{table_name}")
}
