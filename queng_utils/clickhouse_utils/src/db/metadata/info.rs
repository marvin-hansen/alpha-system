use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};

impl Metadata {
    pub(crate) fn metadata_tables(&self) -> [&'static str; 4] {
        DB_TABLES
    }

    pub(crate) fn generate_table_exists_query(&self, table_name: &str) -> String {
        format!("EXISTS TABLE {DB_NAME}.{table_name};")
    }
}
