use crate::db::metadata::{Metadata, TABLE_NAME};

impl Metadata {
    pub fn generate_master_symbols_insert(&self, id: u64, code: &str) -> String {
        let table_name = format!("{TABLE_NAME}.master_symbols");
        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        {id},
        '{code}'
        )
    "
        )
    }
}
