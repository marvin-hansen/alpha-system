use common_config::prelude::ServiceConfig;
use common_exchange::prelude::PortfolioConfig;

use crate::shared::service_ids_to_string;

/// Builds a PostgreSQL query string that updates a service in the database.
///
/// # Args
///
/// * `data` - The service configuration to update.
///
/// # Returns
///
/// A PostgreSQL query string that updates the service in the database.
///
pub fn build_update_service_query(data: &ServiceConfig) -> String {
    format!(
        "UPDATE
                system.service
            SET
                name='{}',
                version={},
                online={},
                description='{}',
                health_check_uri='{}',
                base_uri='{}',
                dependencies='{}',
                exposure={},
                endpoint_name='{}',
                endpoint_version={},
                endpoint_base_uri='{}',
                endpoint_port={},
                endpoint_protocol={},
                metric_uri='{}',
                metric_host='{}',
                metric_port={}
            WHERE
                id={}
            RETURNING service.online",
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        service_ids_to_string(data.dependencies()),
        data.exposure().as_u8(),
        data.service_endpoint().name(),
        data.service_endpoint().version(),
        data.service_endpoint().uri(),
        data.service_endpoint().port(),
        data.service_endpoint().protocol().as_u8(),
        data.metrics_endpoint().uri(),
        data.metrics_endpoint().host(),
        data.metrics_endpoint().port(),
        data.svc_id().as_u8(),
    )
}

pub fn build_update_portfolio_query(data: &PortfolioConfig) -> String {
    format!(
        "UPDATE
                system.portfolio
            SET
                portfolio_id={},
                portfolio_description='{}',
                portfolio_account_type={},
                portfolio_account_id='{}',
                portfolio_currency='{}',
                portfolio_cash={},
                portfolio_margin={},
                portfolio_max_drawdown={},
                instrument_max_allocation={},
                instrument_max_drawdown={},
                portfolio_free_margin={},
                portfolio_free_cash={},
                portfolio_free_margin_percent={},
                portfolio_free_cash_percent={}
            WHERE
                id={}
            RETURNING service.online",
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
        data.portfolio_free_cash_percent(),
        data.portfolio_id()
    )
}
