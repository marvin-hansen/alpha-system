// Working with JSON in ClickHouse
// https://clickhouse.com/docs/en/integrations/data-formats/json

pub(crate) fn generate_all_service_insert(table_name: &str) -> String {
    format!(
        r"
    INSERT INTO {table_name} SELECT * FROM file(services.json,'JSONEachRow')
    "
    )
    .to_string()
}

pub(crate) fn generate_count_services(table_name: &str) -> String {
    format!(
        r"
    SELECT count(*) FROM {table_name}
    "
    )
    .to_string()
}
