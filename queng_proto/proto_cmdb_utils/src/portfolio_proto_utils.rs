use common_exchange::prelude::{AccountType, Instrument, PortfolioConfig};
use proto_cmdb::proto::{ProtoInstrument, ProtoPortfolioConfig};
use std::fmt::Error;

/// Converts a `ProtoPortfolioConfig` into a `PortfolioConfig`.
///
/// This function takes a `ProtoPortfolioConfig` and converts it into a `PortfolioConfig` struct.
/// It extracts the necessary fields from the `ProtoPortfolioConfig` and constructs a new `PortfolioConfig` with them.
///
/// # Errors
///
/// If the conversion of `portfolio_id` or `portfolio_account_type` to their respective types fails,
/// an `std::fmt::Error` is returned.
///
pub fn portfolio_config_from_proto(proto: ProtoPortfolioConfig) -> Result<PortfolioConfig, Error> {
    if proto.portfolio_id < 0 {
        return Err(Error);
    }

    if proto.portfolio_account_type < 0 {
        return Err(Error);
    }

    Ok(PortfolioConfig::new(
        proto.portfolio_id as u32,
        proto.portfolio_description,
        AccountType::from(proto.portfolio_account_type),
        proto.portfolio_account_id,
        proto.portfolio_currency,
        proto.portfolio_cash,
        proto.portfolio_margin,
        proto.portfolio_max_drawdown,
        instrument_from_proto(proto.portfolio_instruments),
        proto.instrument_max_allocation,
        proto.instrument_max_drawdown,
        proto.portfolio_free_margin,
        proto.portfolio_free_cash,
        proto.portfolio_free_margin_percent,
        proto.portfolio_free_cash_percent,
    ))
}

/// Converts a vector of `ProtoInstrument` into a vector of `Instrument`.
///
/// This function iterates over the provided vector of `ProtoInstrument` and converts each element
/// into an `Instrument` struct. It extracts specific fields from each `ProtoInstrument`
/// to construct a new `Instrument`.
///
pub fn instrument_from_proto(proto: Vec<ProtoInstrument>) -> Vec<Instrument> {
    let mut v = Vec::new();

    for p in proto.iter() {
        let i = Instrument::new(
            p.instrument_code.clone(),
            p.instrument_class.clone(),
            p.exchange_code.clone(),
            p.exchange_pair_code.clone(),
            p.base_asset.clone(),
            p.quote_asset.clone(),
            p.instrument_figi.clone(),
        );

        v.push(i);
    }

    v
}

/// Converts a `PortfolioConfig` into a `ProtoPortfolioConfig`.
///
/// This function takes a `PortfolioConfig` reference and converts it into a `ProtoPortfolioConfig` struct.
/// It extracts the necessary fields from the `PortfolioConfig` and constructs a new `ProtoPortfolioConfig` with them.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
pub fn portfolio_config_to_proto(
    portfolio_config: PortfolioConfig,
) -> Result<ProtoPortfolioConfig, Error> {
    Ok(ProtoPortfolioConfig {
        portfolio_id: portfolio_config.portfolio_id() as i32,
        portfolio_description: portfolio_config.portfolio_description().to_string(),
        portfolio_account_type: portfolio_config.portfolio_account_type() as i32,
        portfolio_account_id: portfolio_config.portfolio_account_id().to_string(),
        portfolio_currency: portfolio_config.portfolio_currency().to_string(),
        portfolio_cash: portfolio_config.portfolio_cash(),
        portfolio_margin: portfolio_config.portfolio_margin(),
        portfolio_max_drawdown: portfolio_config.portfolio_max_drawdown(),
        portfolio_instruments: instrument_to_proto(
            portfolio_config.portfolio_instruments().to_owned(),
        ),
        instrument_max_allocation: portfolio_config.instrument_max_allocation(),
        instrument_max_drawdown: portfolio_config.instrument_max_drawdown(),
        portfolio_free_margin: portfolio_config.portfolio_free_margin(),
        portfolio_free_cash: portfolio_config.portfolio_free_cash(),
        portfolio_free_margin_percent: portfolio_config.portfolio_free_margin_percent(),
        portfolio_free_cash_percent: portfolio_config.portfolio_free_cash_percent(),
    })
}

pub fn instrument_to_proto(conf: Vec<Instrument>) -> Vec<ProtoInstrument> {
    let mut v = Vec::new();

    for i in conf.iter() {
        v.push(ProtoInstrument {
            instrument_code: i.code().to_string(),
            instrument_class: i.class().to_string(),
            exchange_code: i.exchange_code().to_string(),
            exchange_pair_code: i.exchange_pair_code().to_string(),
            base_asset: i.base_asset().to_string(),
            quote_asset: i.quote_asset().to_string(),
            instrument_figi: i.instrument_figi().clone(),
        })
    }
    v
}
