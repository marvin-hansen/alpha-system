use crate::db::specs::{Specs, DB_NAME, DB_TABLES};

impl Specs {
    pub(crate) fn specs_tables(&self) -> [&'static str; 1] {
        DB_TABLES
    }

    pub(crate) fn generate_table_exists_query(&self, table_name: &str) -> String {
        format!("EXISTS TABLE {DB_NAME}.{table_name};")
    }
}
