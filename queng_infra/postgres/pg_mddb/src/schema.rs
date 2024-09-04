// @generated automatically by Diesel CLI.

pub mod mddb {
    diesel::table! {
        mddb.assets (asset_code) {
            asset_code -> Varchar,
            asset_name -> Varchar,
            asset_class -> Varchar,
            asset_figi -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        mddb.exchanges (exchanges_code) {
            exchanges_code -> Varchar,
            exchanges_name -> Varchar,
        }
    }

    diesel::table! {
        mddb.instruments (instrument_code) {
            #[max_length = 12]
            instrument_code -> Varchar,
            #[max_length = 12]
            instrument_class -> Varchar,
            instrument_base_asset -> Varchar,
            instrument_quote_asset -> Varchar,
            instrument_exchanges_code -> Varchar,
            instrument_exchange_pair_code -> Varchar,
            instrument_pair_figi -> Nullable<Varchar>,
            instrument_figi -> Nullable<Varchar>,
            trade_start_timestamp -> Int8,
            trade_end_timestamp -> Int8,
        }
    }

    diesel::table! {
        mddb.stats (stats_id) {
            stats_id -> Int4,
            stats_download_timestamp -> Varchar,
            stats_hash -> Varchar,
            stats_assets -> Int4,
            stats_exchanges -> Int4,
            stats_instruments -> Int4,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(assets, exchanges, instruments, stats,);
}
