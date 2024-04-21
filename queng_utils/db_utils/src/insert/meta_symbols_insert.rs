pub fn generate_master_symbols_insert(id: u64, code: &str) -> String {
    let table_name = "default.master_symbols";
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
