use common_config::prelude::ServiceConfig;
use common_pg_queries::pg_inserts::build_insert_portfolio_query;
use common_pg_queries::pg_update::build_update_service_query;
use portfolio_specs::test_portfolio::get_test_portfolio_config;
use smdb_specs::smdb_service_config;

fn get_service_config() -> ServiceConfig {
    smdb_service_config()
}

#[test]
fn test_build_update_service_query() {
    let config = get_service_config();
    let actual_query = build_update_service_query(&config);
    let expected_query = "UPDATE\n                system.service\n            SET\n                name='smdbv1',\n                version=1,\n                online=false,\n                description='SMDB Service Management Database',\n                health_check_uri='smdbv1-service.default.svc.cluster.local:7070/health',\n                base_uri='smdbv1-service.default.svc.cluster.local',\n                dependencies='{3}',\n                exposure=1,\n                endpoint_name='service-registry',\n                endpoint_version=1,\n                endpoint_base_uri='/',\n                endpoint_port=7070,\n                endpoint_protocol=1,\n                metric_uri='metrics',\n                metric_host='0.0.0.0',\n                metric_port=8080\n            WHERE\n                id=1\n            RETURNING service.online".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_update_portfolio_query() {
    let test_data = get_test_portfolio_config();
    let expected_query = "INSERT INTO public.portfolio(portfolio_id, portfolio_description, portfolio_account_type,\n            portfolio_account_id, portfolio_currency, portfolio_cash, portfolio_margin,\n            portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,\n            portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent,\n            portfolio_free_cash_percent)\n            VALUES (1, 'Test portfolio', 1, 'cash_account', 'USD', 1000, 0, 15, 0.05, 10, 0, 1000, 0, 100)\n            RETURNING portfolio_id;".to_string();
    assert_eq!(build_insert_portfolio_query(&test_data), expected_query);
}
