use common_exchange::{AccountType, Instrument, PortfolioConfig};
use proto_cmdb::proto::{ProtoInstrument, ProtoPortfolioConfig};
use std::fmt::Error;

/// Converts a `ProtoPortfolioConfig` into a `PortfolioConfig`.
///
/// This function takes a `ProtoPortfolioConfig` and converts it into a `PortfolioConfig`.
/// It extracts specific fields from the `ProtoPortfolioConfig` to construct a new `PortfolioConfig`.
///
/// # Arguments
///
/// * `proto` - The `ProtoPortfolioConfig` to convert.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
/// # Implementation notes
///
/// This function uses the `instrument_from_proto` function to convert the
/// `portfolio_instruments` into a vector of `Instrument`.
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
/// This function takes a vector of `ProtoInstrument` and converts each element into an `Instrument`.
/// It extracts specific fields from each `ProtoInstrument` to construct a new `Instrument`.
///
/// # Arguments
///
/// * `proto` - A vector of `ProtoInstrument` to convert.
///
/// # Errors
///
/// This function does not return any errors.
///
/// # Implementation notes
///
/// This function iterates over the provided vector of `ProtoInstrument` and converts each element
/// into an `Instrument` struct. It extracts the necessary fields from the `ProtoInstrument`
/// and constructs a new `Instrument` with them.
///
/// The fields that are extracted are:
///
/// - `instrument_code`
/// - `instrument_class`
/// - `exchange_code`
/// - `exchange_pair_code`
/// - `base_asset`
/// - `quote_asset`
/// - `instrument_figi`
///
#[must_use]
pub fn instrument_from_proto(proto: Vec<ProtoInstrument>) -> Vec<Instrument> {
    proto
        .into_iter()
        .map(|p| {
            Instrument::new(
                p.instrument_code,
                p.instrument_class,
                p.exchange_code,
                p.exchange_pair_code,
                p.base_asset,
                p.quote_asset,
                p.instrument_figi,
            )
        })
        .collect()
}

/// Converts a `PortfolioConfig` into a `ProtoPortfolioConfig`.
///
/// This function takes a `PortfolioConfig` and converts it into a `ProtoPortfolioConfig`.
/// It extracts specific fields from the `PortfolioConfig` to construct a new `ProtoPortfolioConfig`.
///
/// # Arguments
///
/// * `portfolio_config` - The `PortfolioConfig` to convert.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
/// # Implementation notes
///
/// This function uses the `instrument_to_proto` function to convert the
/// `portfolio_instruments` into a vector of `ProtoInstrument`.
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

/// Converts a vector of `Instrument` into a vector of `ProtoInstrument`.
///
/// This function takes a vector of `Instrument` and converts each element into a `ProtoInstrument`.
/// It extracts specific fields from each `Instrument` to construct a new `ProtoInstrument`.
///
/// # Arguments
///
/// * `conf` - A vector of `Instrument` to convert.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
/// # Implementation notes
///
/// This function iterates over the provided vector of `Instrument` and converts each element
/// into a `ProtoInstrument` struct. It extracts the necessary fields from the `Instrument`
/// and constructs a new `ProtoInstrument` with them.
///
/// The fields that are extracted are:
///
/// - `instrument_code`
/// - `instrument_class`
/// - `exchange_code`
/// - `exchange_pair_code`
/// - `base_asset`
/// - `quote_asset`
/// - `instrument_figi`
///
#[must_use]
pub fn instrument_to_proto(conf: Vec<Instrument>) -> Vec<ProtoInstrument> {
    let mut v = Vec::new();

    for i in &conf {
        v.push(ProtoInstrument {
            instrument_code: i.code().to_string(),
            instrument_class: i.class().to_string(),
            exchange_code: i.exchange_code().to_string(),
            exchange_pair_code: i.exchange_pair_code().to_string(),
            base_asset: i.base_asset().to_string(),
            quote_asset: i.quote_asset().to_string(),
            instrument_figi: i.instrument_figi().clone(),
        });
    }
    v
}
