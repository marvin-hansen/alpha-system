use common::prelude::{AccountType, ExchangeID, PortfolioConfig};
use proto::binding::ProtoPortfolioConfig;
use std::fmt::Error;

pub fn portfolio_config_from_proto(proto: ProtoPortfolioConfig) -> Result<PortfolioConfig, Error> {
    Ok(PortfolioConfig::new(
        proto.portfolio_id as u32,
        proto.portfolio_description,
        AccountType::from(proto.portfolio_account_type),
        proto.portfolio_account_id,
        ExchangeID::from(proto.portfolio_exchange_id),
        proto.portfolio_currency,
        proto.portfolio_cash,
        proto.portfolio_margin,
        proto.portfolio_max_drawdown,
        proto.portfolio_instruments.into_iter().collect(),
        proto.instrument_max_allocation,
        proto.instrument_max_drawdown,
        proto.portfolio_free_margin,
        proto.portfolio_free_cash,
        proto.portfolio_free_margin_percent,
        proto.portfolio_free_cash_percent,
    ))
}

pub fn portfolio_config_to_proto(
    portfolio_config: PortfolioConfig,
) -> Result<ProtoPortfolioConfig, Error> {
    Ok(ProtoPortfolioConfig {
        portfolio_id: portfolio_config.portfolio_id() as i32,
        portfolio_description: portfolio_config.portfolio_description().to_string(),
        portfolio_account_type: portfolio_config.portfolio_account_type() as i32,
        portfolio_account_id: portfolio_config.portfolio_account_id().to_string(),
        portfolio_exchange_id: portfolio_config.portfolio_exchange_id() as i32,
        portfolio_currency: portfolio_config.portfolio_currency().to_string(),
        portfolio_cash: portfolio_config.portfolio_cash(),
        portfolio_margin: portfolio_config.portfolio_margin(),
        portfolio_max_drawdown: portfolio_config.portfolio_max_drawdown(),
        portfolio_instruments: portfolio_config.portfolio_instruments().to_owned(),
        instrument_max_allocation: portfolio_config.instrument_max_allocation(),
        instrument_max_drawdown: portfolio_config.instrument_max_drawdown(),
        portfolio_free_margin: portfolio_config.portfolio_free_margin(),
        portfolio_free_cash: portfolio_config.portfolio_free_cash(),
        portfolio_free_margin_percent: portfolio_config.portfolio_free_margin_percent(),
        portfolio_free_cash_percent: portfolio_config.portfolio_free_cash_percent(),
    })
}
