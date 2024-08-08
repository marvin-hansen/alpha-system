use common_pg_queries::portfolio_insert::build_insert_portfolio_query;
use portfolio_specs::test_portfolio::get_test_portfolio_config;

#[test]
fn test_build_insert_portfolio_query() {
    let test_data = get_test_portfolio_config();
    let expected_query = "INSERT INTO public.portfolio(portfolio_id, portfolio_description, portfolio_account_type,\n            portfolio_account_id, portfolio_currency, portfolio_cash, portfolio_margin,\n            portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,\n            portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent,\n            portfolio_free_cash_percent)\n            VALUES (1, 'Test portfolio', 1, 'cash_account', 'USD', 1000, 0, 15, 0.05, 10, 0, 1000, 0, 100)\n            RETURNING portfolio_id;".to_string();
    assert_eq!(build_insert_portfolio_query(&test_data), expected_query);
}
