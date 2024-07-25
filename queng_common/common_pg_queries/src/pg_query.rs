use common_config::prelude::ServiceID;

pub fn generate_count_table_query(schema_name: &str, table_name: &str) -> String {
    format!("SELECT COUNT(*) FROM {schema_name}.{table_name};")
}

/// Builds a PostgreSQL query to check if a service ID exists in the database.
///
/// # Arguments
///
/// * `id` - The service ID to check for existence.
///
/// # Returns
///
/// A PostgreSQL query string that checks if the service ID exists in the database.
pub fn build_check_if_service_id_exists_query(id: &ServiceID) -> String {
    format!(
        "
        SELECT EXISTS (
        SELECT
            id
        FROM
            system.service
        WHERE
            id={}
        )
        ",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to check if a service ID is online in the database.
///
/// # Arguments
///
/// * `id` - The service ID to check for online status.
///
/// # Returns
///
/// A PostgreSQL query string that checks if the service ID is online in the database.
pub fn build_check_if_service_id_online_query(id: &ServiceID) -> String {
    format!(
        "
        SELECT EXISTS (
        SELECT
            id, online
        FROM
            system.service
        WHERE
            id={}
        AND
            online=true
        )
        ",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to set the online status of a service in the database.
///
/// # Arguments
///
/// * `id` - The service ID to set the online status for.
/// * `online` - The online status to set.
///
/// # Returns
///
/// A PostgreSQL query string that sets the online status of the service in the database.
pub fn build_set_svc_online_query(id: &ServiceID, online: bool) -> String {
    format!(
        "
        UPDATE
            system.service
        SET
            online={}
        WHERE
            id={}
        RETURNING service.online
        ",
        online,
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to select a service by ID from the database.
///
/// # Arguments
///
/// * `id` - The service ID to select.
///
/// # Returns
///
/// A PostgreSQL query string that selects the service by ID from the database.
pub fn build_read_service_by_id_query(id: &ServiceID) -> String {
    format!(
        "SELECT
                id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,
                endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
                metric_uri, metric_host, metric_port
            FROM
                system.service
            WHERE
                id={}",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to select all services from the database.
///
/// # Returns
///
/// A PostgreSQL query string that selects all services from the database.
pub fn build_read_all_services_query() -> String {
    "SELECT
             id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,
             endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
             metric_uri, metric_host, metric_port
         FROM
           system.service
         ORDER BY
            id
  ".to_string()
}

/// Builds a PostgreSQL query to delete a service by ID from the database.
///
/// # Arguments
///
/// * `id` - The service ID to delete.
///
/// # Returns
///
/// A PostgreSQL query string that deletes the service by ID from the database.
pub fn build_delete_service_query(id: &ServiceID) -> String {
    format!(
        "DELETE FROM system.service
             WHERE
                id={}",
        id.as_u8()
    )
}

pub fn build_check_if_portfolio_id_exists_query(portfolio_id: u16) -> String {
    format!(
        "
        SELECT EXISTS (
        SELECT
            portfolio_id
        FROM
            public.portfolio
        WHERE
            portfolio_id={}
        )
        ",
        portfolio_id
    )
}

pub fn build_delete_portfolio_query(portfolio_id: u16) -> String {
    format!(
        "DELETE FROM
                public.portfolio
             WHERE
                portfolio_id={}
                ",
        portfolio_id
    )
}

pub fn build_query_portfolio_by_id(id: u16) -> String {
    format!(
        "SELECT
                portfolio_id,
                portfolio_description,
                portfolio_account_type,
                portfolio_account_id,
                portfolio_currency,
                portfolio_cash,
                portfolio_margin, portfolio_max_drawdown,
                instrument_max_allocation,
                instrument_max_drawdown,
                portfolio_free_margin,
                portfolio_free_cash,
                portfolio_free_margin_percent,
                portfolio_free_cash_percent

            FROM
                public.portfolio
            WHERE
                portfolio_id={}
                ;",
        id
    )
}

pub fn build_query_instrument_ids_by_portfolio_id(portfolio_id: u16) -> String {
    format!(
        "SELECT
            instrument_id
        FROM
             public.portfolio_instrument
        WHERE
            portfolio_id = {}",
        portfolio_id
    )
}

pub fn build_query_instruments_by_ids(instrument_ids: &Vec<i32>) -> String {
    format!(
        "SELECT
            id, code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi
        FROM
            public.instrument
        WHERE
            id IN ({})
            ;",
        instrument_ids
            .iter()
            .map(|id| format!("{}", id))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
