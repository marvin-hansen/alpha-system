use crate::types::SymbolMetaData;

pub fn generate_meta_data_insert_query(
    meta_data_table: &str,
    meta_data: &SymbolMetaData,
) -> String {
    let symbol = meta_data.symbol();
    let symbol_id = meta_data.symbol_id();
    let table_name = meta_data.table_name();
    let number_of_rows = meta_data.number_of_rows();

    format!(
        r"
        INSERT INTO {meta_data_table}  VALUES ('{symbol}', {symbol_id},' {table_name}', {number_of_rows}) ;
    "
    )
}
