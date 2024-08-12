// @generated automatically by Diesel CLI.

pub mod cmdb {
    diesel::table! {
        cmdb.instrument (code) {
            code -> Varchar,
            class -> Varchar,
            exchange_code -> Varchar,
            exchange_pair_code -> Varchar,
            base_asset -> Varchar,
            quote_asset -> Varchar,
            instrument_figi -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        cmdb.portfolio (portfolio_id) {
            portfolio_id -> Int4,
            portfolio_description -> Varchar,
            portfolio_account_type -> Int4,
            portfolio_account_id -> Varchar,
            portfolio_currency -> Varchar,
            portfolio_cash -> Float8,
            portfolio_margin -> Float8,
            portfolio_max_drawdown -> Float8,
            instrument_max_allocation -> Float8,
            instrument_max_drawdown -> Float8,
            portfolio_free_margin -> Float8,
            portfolio_free_cash -> Float8,
            portfolio_free_margin_percent -> Float8,
            portfolio_free_cash_percent -> Float8,
        }
    }

    diesel::table! {
        cmdb.portfolio_instrument (portfolio_id, instrument_id) {
            portfolio_id -> Int4,
            instrument_id -> Varchar,
        }
    }

    diesel::joinable!(portfolio_instrument -> instrument (instrument_id));
    diesel::joinable!(portfolio_instrument -> portfolio (portfolio_id));

    diesel::allow_tables_to_appear_in_same_query!(instrument, portfolio, portfolio_instrument,);
}
