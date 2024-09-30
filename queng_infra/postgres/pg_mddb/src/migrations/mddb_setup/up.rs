pub const SCHEMA_UP: &'static str = r#"
-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS mddb;

CREATE TABLE mddb.assets(
	"asset_code" VARCHAR NOT NULL PRIMARY KEY,
	"asset_name" VARCHAR NOT NULL,
	"asset_classes" text[] NOT NULL,
	"asset_figi" VARCHAR
);

CREATE TABLE mddb.exchanges(
	"exchange_id" VARCHAR NOT NULL PRIMARY KEY,
	"exchange_name" VARCHAR NOT NULL
);

CREATE TABLE mddb.instruments(
	"instrument_id" VARCHAR NOT NULL PRIMARY KEY,
	"instrument_class" VARCHAR NOT NULL,
	"instrument_base_asset" VARCHAR NOT NULL,
	"instrument_quote_asset" VARCHAR NOT NULL,
    "instrument_exchanges_code" VARCHAR NOT NULL,
    "instrument_exchange_pair_code" VARCHAR NOT NULL,
	"instrument_pair_figi" VARCHAR,
	"instrument_figi" VARCHAR,
	"instrument_trade_start_timestamp" BIGINT,
	"instrument_trade_end_timestamp" BIGINT
);

-- FIGI (Financial Instrument Global Identifier) is a unique identifier for an instrument similar to ISIN

-- Instrument relations
-- instruments has exchanges (via exchange code)
-- instruments has assets (base and quote) via asset code

CREATE TABLE mddb.instruments_exchanges (
    instrument_id VARCHAR REFERENCES mddb.instruments(instrument_id),
    exchange_id   VARCHAR REFERENCES mddb.exchanges(exchange_id),
    PRIMARY KEY (instrument_id, exchange_id)
);

-- Download statistics i.e. when was the last download and how many assets, instruments etc.
-- Hash helps to compare if the data has changed.
CREATE TABLE mddb.stats(
    "stats_id" INTEGER NOT NULL PRIMARY KEY,
    "stats_hash" VARCHAR NOT NULL,
    "stats_download_timestamp" VARCHAR NOT NULL,
    "stats_number_assets" INTEGER NOT NULL,
    "stats_number_exchanges" INTEGER NOT NULL,
    "stats_number_instruments" INTEGER NOT NULL
);
"#;
