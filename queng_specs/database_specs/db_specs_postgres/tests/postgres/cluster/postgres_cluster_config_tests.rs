use db_specs_postgres::postgres::get_cluster_db_config;

#[test]
fn test_get_cluster_db_config() {
    let config = get_cluster_db_config(
        String::from("username"),
        String::from("password"),
        String::from("prod_db"),
    );

    assert_eq!(
        config.pg_host(),
        "postgres-cluster-rw.default.svc.cluster.local"
    );
    assert_eq!(config.pg_user(), "username");
    assert_eq!(config.pg_password(), "password");
    assert_eq!(config.pg_database(), "prod_db");
    assert_eq!(config.pg_port(), 5432);
    assert_eq!(config.pg_max_connections(), 10);
}
