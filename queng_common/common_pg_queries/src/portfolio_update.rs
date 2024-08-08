use common_exchange::prelude::PortfolioConfig;

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
