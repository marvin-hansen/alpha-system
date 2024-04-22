// Working with JSON in ClickHouse
// https://clickhouse.com/docs/en/integrations/data-formats/json

pub fn generate_all_service_insert() -> String {
    r"
    INSERT INTO system.services SELECT * FROM file(services.json,'JSONEachRow')
    "
    .to_string()
}
