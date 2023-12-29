use common::prelude::DBConfig;
use db_specs::prelude::{db_config_ci, db_config_cluster, db_config_local};

const PORT: u16 = 8000;

#[test]
fn test_db_config_local() {
    let expected = DBConfig::new(
        PORT,
        "0.0.0.0".to_string(),
        "test".to_string(),
        "test".to_string(),
        "root".to_string(),
        "root".to_string(),
    );

    let actual = db_config_local();

    assert_eq!(expected, actual);
}

#[test]
fn test_db_config_ci() {
    let expected = DBConfig::default();
    let actual = db_config_ci();

    assert_eq!(expected, actual);
}

#[test]
fn test_db_config_cluster() {
    let expected = DBConfig::new(
        PORT,
        "db.namespace.url.cluster".to_string(),
        "cluster".to_string(),
        "service".to_string(),
        "root".to_string(),
        "root".to_string(),
    );

    let actual = db_config_cluster();

    assert_eq!(expected, actual);
}
