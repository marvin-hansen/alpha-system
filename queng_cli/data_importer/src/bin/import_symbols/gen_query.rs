use lib_import::types::assets::{Asset, Metadata};
use lib_import::types::exchanges::Exchange;
use lib_import::types::instruments::{Instrument, InstrumentMetadata};

pub fn generate_asset_insert(asset: &Asset) -> String {
    let table_name = "default.assets";
    let code = &asset.code;
    let name = &asset.name;
    let asset_class = &asset.asset_class;
    let asset_figi = extract_asset_figi(&asset.metadata);

    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
        '{asset_class}',
        '{asset_figi}'
        );
    "
    )
}

fn extract_asset_figi(metadata: &Option<Metadata>) -> String {
    let empty_string = "".to_string();
    let asset_figi = match metadata {
        Some(metadata) => match &metadata.asset_figi {
            Some(figi) => figi.to_owned(),
            None => empty_string,
        },
        None => empty_string,
    };

    asset_figi
}

pub fn generate_exchange_insert(exchange: &Exchange) -> String {
    let table_name = "default.exchanges";
    let code = &exchange.code;
    let name = &exchange.name;
    let active = exchange.active;
    let url = &exchange.url.clone().unwrap_or("".to_string());
    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
         {active},
         '{url}'
         );
    "
    )
}

pub fn generate_instruments_insert(instrument: &Instrument) -> String {
    let table_name = "default.instruments";
    let trade_start_timestamp = instrument.trade_start_timestamp.unwrap_or(0);
    let trade_end_timestamp = instrument.trade_end_timestamp.unwrap_or(0);
    let exchange_code = instrument.exchange_code();
    let exchange_pair_code = &instrument.exchange_pair_code;
    let base_asset = &instrument.base_asset;
    let quote_asset = &instrument.quote_asset;
    let code = &instrument.code;
    let class = &instrument.class;
    let (pair_figi, instrument_figi) = extract_instrument_figi(&instrument.metadata);

    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        {trade_start_timestamp},
        {trade_end_timestamp},
        '{exchange_code}',
        '{exchange_pair_code}',
        '{base_asset}',
        '{quote_asset}',
        '{code}',
        '{class}',
        '{pair_figi}',
        '{instrument_figi}'
        "
    )
    .to_string()
}

fn extract_instrument_figi(metadata: &Option<InstrumentMetadata>) -> (String, String) {
    let empty_string = "".to_string();

    (empty_string.clone(), empty_string.clone())
}
