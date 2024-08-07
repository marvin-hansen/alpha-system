use common_config::prelude::ServiceConfig;
use common_exchange::prelude::{Instrument, PortfolioConfig};

use crate::shared;

/// Builds the SQL query for inserting a service into the system.service table.
///
/// This method takes a `ServiceConfig` object and generates an SQL query string
/// to insert a new service into the system.service table. The query includes all
/// the fields of the `ServiceConfig` object, including the service ID, name,
/// version, online status, description, health check URI, base URI, dependencies,
/// exposure level, endpoint, and metrics.
///
/// # Arguments
///
/// * `data` - A reference to a `ServiceConfig` object from which a SQL query is generated.
///
/// # Returns
///
/// Returns a `String` containing the SQL query for inserting the service.
///
pub fn build_insert_service_query(data: &ServiceConfig) -> String {
    format!(
        "INSERT INTO system.service(id, name, version, online, description, health_check_uri,
            base_uri, dependencies, exposure,
            endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
            metric_uri, metric_host, metric_port)
             VALUES({}, '{}', {}, {}, '{}', '{}', '{}', '{}', {},
                '{}', {}, '{}', {}, {},
                '{}', '{}', {}
            )
            RETURNING id",
        data.svc_id().as_u8(),
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        shared::service_ids_to_string(data.dependencies()),
        data.exposure().as_u8(),
        data.service_endpoint().name(),
        data.service_endpoint().version(),
        data.service_endpoint().uri(),
        data.service_endpoint().port(),
        data.service_endpoint().protocol().as_u8(),
        data.metrics_endpoint().uri(),
        data.metrics_endpoint().host(),
        data.metrics_endpoint().port()
    )
}

// insert into portfolio(portfolio_id,portfolio_description,portfolio_account_type, portfolio_account_id, portfolio_currency,
// portfolio_cash, portfolio_margin, portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,
// portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent, portfolio_free_cash_percent)
// VALUES(
// 1,
// 'cash portfolio',
// 2,
// 'cash_account',
// 'USD',
// 1000.0,
// 0.0,
// 20.0,
// 5.0,
// 10.0,
// 0.0,
// 1000.0,
// 0.0,
// 100.0
// )
// RETURNING portfolio_id;
pub fn build_insert_portfolio_query(data: &PortfolioConfig) -> String {
    format!(
        "INSERT INTO public.portfolio(portfolio_id, portfolio_description, portfolio_account_type,
            portfolio_account_id, portfolio_currency, portfolio_cash, portfolio_margin,
            portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,
            portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent,
            portfolio_free_cash_percent)
            VALUES ({}, '{}', {}, '{}', '{}', {}, {}, {}, {}, {}, {}, {}, {}, {})
            RETURNING portfolio_id;",
        data.portfolio_id(),
        data.portfolio_description(),
        data.portfolio_account_type().as_u8(),
        data.portfolio_account_id(),
        data.portfolio_currency(),
        data.portfolio_cash(),
        data.portfolio_margin(),
        data.portfolio_max_drawdown(),
        data.instrument_max_allocation(),
        data.instrument_max_drawdown(),
        data.portfolio_free_margin(),
        data.portfolio_free_cash(),
        data.portfolio_free_margin_percent(),
        data.portfolio_free_cash_percent()
    )
}

// insert into public.instrument(id,code ,"class" ,exchange_code,exchange_pair_code,base_asset,quote_asset,instrument_figi )
// VALUES(
// 1,
// 'ens-krw',
// 'spot',
// 'cbse',
// 'KRW-ENS',
// 'ens',
// 'krw',
// null
// )
// RETURNING id;
pub fn build_insert_instrument_query(data: &Instrument) -> String {
    format!(
        "INSERT INTO public.instrument(code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi)
             VALUES('{}', '{}', '{}', '{}', '{}', '{}', '{}')
             RETURNING code",
        data.code(), data.class(), data.exchange_code(), data.exchange_pair_code(), data.base_asset(), data.quote_asset(), data.instrument_figi().clone().unwrap_or_else(|| "null".to_string())
    )
}

pub fn build_insert_portfolio_instrument_query(portfolio_id: u64, instrument_id: String) -> String {
    format!(
        "INSERT INTO public.portfolio_instrument (portfolio_id, instrument_id) VALUES ({}, '{}')",
        portfolio_id, instrument_id
    )
}
