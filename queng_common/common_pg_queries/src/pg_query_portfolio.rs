/// Builds a SQL query that checks if a portfolio with the given ID exists.
///
/// # Arguments
///
/// * `portfolio_id` - The ID of the portfolio to check.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_check_if_portfolio_id_exists_query(portfolio_id: u16) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            portfolio_id
        FROM
            public.portfolio
        WHERE
            portfolio_id={}
        )",
        portfolio_id
    )
}

/// Builds a SQL query that checks if an instrument with the given ID exists.
///
/// # Arguments
///
/// * `instrument_id` - The ID of the instrument to check.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_check_if_instrument_id_exists_query(instrument_id: &str) -> String {
    format!(
        "SELECT EXISTS (
            SELECT
                code
            FROM
                public.instrument
             where
                code='{}'
        );",
        instrument_id
    )
}

/// Builds a SQL query that retrieves the instrument ID if it exists.
///
/// # Arguments
///
/// * `instrument_code` - The code of the instrument to retrieve.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_get_instrument_id_if_exists_query(instrument_code: &str) -> String {
    format!(
        "SELECT
            id
        FROM
            instrument
        where
           EXISTS (
             SELECT
                    code
                FROM
                    public.instrument
                WHERE
                    code={instrument_code}
                );",
    )
}

/// Builds a SQL query that deletes all rows from the `portfolio_instrument`
/// table where the `portfolio_id` matches the given `portfolio_id`.
///
/// # Arguments
///
/// * `portfolio_id` - The ID of the portfolio to delete rows from.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_delete_portfolio_instrument_query(portfolio_id: u16) -> String {
    format!(
        "DELETE FROM public.portfolio_instrument
             WHERE
                portfolio_id={}",
        portfolio_id
    )
}

/// Builds a SQL query that deletes a portfolio from the `portfolio` table by its ID.
///
/// # Arguments
///
/// * `portfolio_id` - The ID of the portfolio to delete.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_delete_portfolio_query(portfolio_id: u16) -> String {
    format!(
        "DELETE FROM public.portfolio
             WHERE
                portfolio_id={}",
        portfolio_id
    )
}

/// Builds a SQL query that retrieves a portfolio from the `portfolio` table by its ID.
///
/// # Arguments
///
/// * `id` - The ID of the portfolio to retrieve.
///
/// # Returns
///
/// The SQL query as a string.
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
                portfolio_id={};",
        id
    )
}

/// Builds a SQL query that retrieves the instrument IDs associated with a portfolio by its ID.
///
/// # Arguments
///
/// * `portfolio_id` - The ID of the portfolio to retrieve the instrument IDs for.
///
/// # Returns
///
/// The SQL query as a string.
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

/// Builds a SQL query that retrieves instruments by their IDs.
///
/// # Arguments
///
/// * `instrument_ids` - The IDs of the instruments to retrieve.
///
/// # Returns
///
/// The SQL query as a string.
pub fn build_query_instruments_by_ids(instrument_ids: &[String]) -> String {
    format!(
        "SELECT
            code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi
        FROM
            public.instrument
        WHERE
            code IN ({})
            ;",
        instrument_ids
            .iter()
            .map(|id| format!("'{}'", id))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
