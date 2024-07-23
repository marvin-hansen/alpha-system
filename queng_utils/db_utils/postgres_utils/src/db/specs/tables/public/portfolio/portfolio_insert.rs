use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use common_exchange::prelude::PortfolioConfig;

impl Specs {
    // insert into portfolio(portfolio_id,portfolio_description,portfolio_account_type, portfolio_account_id, portfolio_currency,
    // portfolio_cash, portfolio_margin, portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,
    // portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent, portfolio_free_cash_percent)
    // VALUES(
    // 1,
    // 'cash portfolio',
    // 'Spot',
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
    pub async fn insert_portfolio(&self, data: &PortfolioConfig) -> Result<(), PostgresUtilError> {
        let query = self.build_insert_portfolio_query(data);
        let portfolio_id = match self.execute_insert_query(&query).await {
            Ok(id) => id,
            Err(err) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to insert portfolio: {}",
                    err
                )))
            }
        };

        for instrument in data.portfolio_instruments() {
            let instrument_id = match self.insert_instrument(instrument).await {
                Ok(id) => id,
                Err(err) => {
                    return Err(PostgresUtilError::new(format!(
                        "Failed to insert instrument: {}",
                        err
                    )))
                }
            };

            match self
                .insert_portfolio_instrument(portfolio_id, instrument_id)
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    return Err(PostgresUtilError::new(format!(
                        "Failed to insert portfolio_instrument: {}",
                        err
                    )))
                }
            };
        }

        Ok(())
    }

    fn build_insert_portfolio_query(&self, data: &PortfolioConfig) -> String {
        format!(
            "INSERT INTO portfolio(portfolio_id, portfolio_description, portfolio_account_type,
            portfolio_account_id, portfolio_currency, portfolio_cash, portfolio_margin,
            portfolio_max_drawdown, instrument_max_allocation, instrument_max_drawdown,
            portfolio_free_margin, portfolio_free_cash, portfolio_free_margin_percent,
            portfolio_free_cash_percent)
            VALUES ({}, '{}', '{}', '{}', '{}', {}, {}, {}, {}, {}, {}, {}, {}, {})
            RETURNING portfolio_id;",
            data.portfolio_id(),
            data.portfolio_description(),
            data.portfolio_account_type(),
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
}
