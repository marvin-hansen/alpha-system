// @generated automatically by Diesel CLI.

pub mod mddb {
    diesel::table! {
        mddb.assets (asset_code) {
            asset_code -> Varchar,
            asset_hash -> Varchar,
            asset_name -> Varchar,
            asset_classes -> Array<Nullable<Text>>,
            asset_figi -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        mddb.exchanges (exchange_id) {
            exchange_id -> Varchar,
            exchange_hash -> Varchar,
            exchange_name -> Varchar,
        }
    }

    diesel::table! {
        mddb.instruments (instrument_id) {
            instrument_id -> Varchar,
            instrument_hash -> Varchar,
            instrument_class -> Varchar,
            instrument_base_asset -> Varchar,
            instrument_quote_asset -> Varchar,
            instrument_exchanges_code -> Varchar,
            instrument_exchange_pair_code -> Varchar,
            instrument_pair_figi -> Nullable<Varchar>,
            instrument_figi -> Nullable<Varchar>,
            instrument_trade_start_timestamp -> Nullable<Int8>,
            instrument_trade_end_timestamp -> Nullable<Int8>,
        }
    }

    diesel::table! {
        mddb.instruments_exchanges (instrument_id, exchange_id) {
            instrument_id -> Varchar,
            exchange_id -> Varchar,
        }
    }

    diesel::table! {
        mddb.stats (stats_id) {
            stats_id -> Int4,
            stats_hash -> Varchar,
            stats_download_timestamp -> Varchar,
            stats_number_assets -> Int4,
            stats_number_exchanges -> Int4,
            stats_number_instruments -> Int4,
        }
    }

    diesel::joinable!(instruments_exchanges -> exchanges (exchange_id));
    diesel::joinable!(instruments_exchanges -> instruments (instrument_id));

    diesel::allow_tables_to_appear_in_same_query!(
        assets,
        exchanges,
        instruments,
        instruments_exchanges,
        stats,
    );
}
