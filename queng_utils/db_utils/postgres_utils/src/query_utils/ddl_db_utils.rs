pub(crate) fn generate_create_db_ddl(db_name: &str) -> String {
    format!("CREATE DATABASE IF NOT EXISTS {db_name};")
}

pub(crate) fn generate_drop_db_ddl(db_name: &str) -> String {
    format!("DROP DATABASE IF EXISTS {db_name};")
}
