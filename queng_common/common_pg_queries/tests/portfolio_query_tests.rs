use common_pg_queries::portfolio_query::{
    build_check_if_instrument_id_exists_query, build_check_if_portfolio_id_exists_query,
    build_delete_portfolio_instrument_query, build_delete_portfolio_query,
    build_get_instrument_id_if_exists_query, build_query_instruments_by_ids,
    build_query_portfolio_by_id,
};

#[test]
fn test_build_check_if_portfolio_id_exists_query() {
    let portfolio_id = 1;
    let actual_query = build_check_if_portfolio_id_exists_query(portfolio_id);
    let expected_query = "SELECT EXISTS (
        SELECT
            portfolio_id
        FROM
            public.portfolio
        WHERE
            portfolio_id=1
        )"
    .to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_check_if_instrument_id_exists_query() {
    let instrument_id = "BTCUSD";
    let actual_query = build_check_if_instrument_id_exists_query(instrument_id);
    let expected_query = "SELECT EXISTS (
            SELECT
                code
            FROM
                public.instrument
             where
                code='BTCUSD'
        );"
    .to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_get_instrument_id_if_exists_query() {
    let instrument_code = "BTCUSD";
    let actual_query = build_get_instrument_id_if_exists_query(instrument_code);
    let expected_query = "SELECT\n            id\n        FROM\n            instrument\n        where\n           EXISTS (\n             SELECT\n                    code\n                FROM\n                    public.instrument\n                WHERE\n                    code=BTCUSD\n                );".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_delete_portfolio_instrument_query() {
    let portfolio_id = 1;
    let actual_query = build_delete_portfolio_instrument_query(portfolio_id);
    let expected_query = "DELETE FROM public.portfolio_instrument\n             WHERE\n                portfolio_id=1".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_delete_portfolio_query() {
    let portfolio_id = 1;
    let actual_query = build_delete_portfolio_query(portfolio_id);
    let expected_query =
        "DELETE FROM public.portfolio\n             WHERE\n                portfolio_id=1"
            .to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_query_portfolio_by_id() {
    let portfolio_id = 1;
    let actual_query = build_query_portfolio_by_id(portfolio_id);
    let expected_query = "SELECT\n                portfolio_id,\n                portfolio_description,\n                portfolio_account_type,\n                portfolio_account_id,\n                portfolio_currency,\n                portfolio_cash,\n                portfolio_margin, portfolio_max_drawdown,\n                instrument_max_allocation,\n                instrument_max_drawdown,\n                portfolio_free_margin,\n                portfolio_free_cash,\n                portfolio_free_margin_percent,\n                portfolio_free_cash_percent\n            FROM\n                public.portfolio\n            WHERE\n                portfolio_id=1;".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_query_instrument_ids_by_portfolio_id() {
    let portfolio_id = 1;
    let actual_query = build_query_portfolio_by_id(portfolio_id);
    let expected_query = "SELECT\n                portfolio_id,\n                portfolio_description,\n                portfolio_account_type,\n                portfolio_account_id,\n                portfolio_currency,\n                portfolio_cash,\n                portfolio_margin, portfolio_max_drawdown,\n                instrument_max_allocation,\n                instrument_max_drawdown,\n                portfolio_free_margin,\n                portfolio_free_cash,\n                portfolio_free_margin_percent,\n                portfolio_free_cash_percent\n            FROM\n                public.portfolio\n            WHERE\n                portfolio_id=1;".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_query_instruments_by_ids() {
    let instrument_ids = vec!["BTCUSD".to_string()];
    let actual_query = build_query_instruments_by_ids(&instrument_ids);
    let expected_query = "SELECT
            code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi
        FROM
            public.instrument
        WHERE
            code IN ('BTCUSD')
            ;"
    .to_string();
    assert_eq!(actual_query, expected_query);
}
