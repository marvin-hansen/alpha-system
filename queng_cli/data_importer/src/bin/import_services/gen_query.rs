// Working with JSON in ClickHouse
// https://clickhouse.com/docs/en/integrations/data-formats/json

pub(crate) fn generate_all_service_insert() -> String {
    let table_name = "default.services";
    format!(
        r"
    INSERT INTO {table_name} SELECT * FROM file(services.json,'JSONEachRow')
    "
    )
    .to_string()
}

pub(crate) fn generate_count_services() -> String {
    r"
    SELECT count(*) FROM system.services
    "
    .to_string()
}
