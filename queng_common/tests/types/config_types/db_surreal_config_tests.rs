use common::prelude::SurrealDBConfig;

fn get_db_config() -> SurrealDBConfig {
    SurrealDBConfig::new_connection(27017, "localhost".to_string())
}

#[test]
fn test_new() {
    let config = get_db_config();
    assert_eq!(config.port(), 27017);
    assert_eq!(config.host(), "localhost");
}
