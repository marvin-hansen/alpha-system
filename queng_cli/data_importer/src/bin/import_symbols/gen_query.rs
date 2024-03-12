use lib_import::types::assets::{Asset, Metadata};
use lib_import::types::exchanges::Exchange;

pub fn generate_asset_insert(asset: &Asset) -> String {
    let table_name = "default.assets";
    let code = asset.code.clone();
    let name = asset.name.clone();
    let asset_class = asset.asset_class.clone();
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
    let code = exchange.code.clone();
    let name = exchange.name.clone();
    let active = exchange.active;
    let url = exchange.url.clone().unwrap_or("".to_string());
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
