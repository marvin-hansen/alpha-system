pub fn generate_trades_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    format!(
        r"
        INSERT INTO {table_name} (timestamp, price, volume)
        SELECT timestamp, price, volume
        FROM
        file('{path}', 'CSV', 'timestamp Datetime64(3), price Float64, volume Float64')
        "
    )
}
