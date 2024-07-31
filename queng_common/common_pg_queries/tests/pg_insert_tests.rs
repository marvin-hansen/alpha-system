use common_config::prelude::ServiceConfig;
use common_pg_queries::pg_inserts::{build_insert_portfolio_query, build_insert_service_query};
use portfolio_specs::test_portfolio::get_test_portfolio_config;
use smdb_specs::smdb_service_config;

fn get_service_config() -> ServiceConfig {
    smdb_service_config()
}

#[test]
fn test_build_insert_service_query() {
    let config = get_service_config();
    let query = build_insert_service_query(&config);
    let expected_query = "INSERT INTO system.service(id, name, version, online, description, health_check_uri,\n            base_uri, dependencies, exposure,\n            endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,\n            metric_uri, metric_host, metric_port)\n             VALUES(1, 'smdbv1', 1, false, 'SMDB Service Management Database', 'smdbv1-service.default.svc.cluster.local:7070/health', 'smdbv1-service.default.svc.cluster.local', '{3}', 1,\n                'service-registry', 1, '/', 7070, 1,\n                'metrics', '0.0.0.0', 8080\n            )\n            RETURNING id".to_string();
    assert_eq!(query, expected_query);
}

#[test]
fn test_build_insert_portfolio_query() {
    let test_data = get_test_portfolio_config();
    let expected_query = "INSERT INTO public.portfolio(portfolio_id, portfolio_description, portfolio_account_type,\n            portfolio_account_id, portfolio_currency, portfolio_cash, portfolio_margin,\n            portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,\n            portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent,\n            portfolio_free_cash_percent)\n            VALUES (1, 'Test portfolio', 1, 'cash_account', 'USD', 1000, 0, 15, 0.05, 10, 0, 1000, 0, 100)\n            RETURNING portfolio_id;".to_string();
    assert_eq!(build_insert_portfolio_query(&test_data), expected_query);
}
