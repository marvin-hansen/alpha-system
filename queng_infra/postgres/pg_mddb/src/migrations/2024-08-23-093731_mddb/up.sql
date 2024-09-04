-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS mddb;

CREATE TABLE mddb.assets(
	"asset_code" VARCHAR NOT NULL PRIMARY KEY,
	"asset_name" VARCHAR NOT NULL,
	"asset_class" VARCHAR NOT NULL,
	"asset_figi" VARCHAR
);

CREATE TABLE mddb.exchanges(
	"exchanges_code" VARCHAR NOT NULL PRIMARY KEY,
	"exchanges_name" VARCHAR NOT NULL
);

CREATE TABLE mddb.instruments(
	"instrument_code" VARCHAR (12) NOT NULL PRIMARY KEY,
	"instrument_class" VARCHAR (12) NOT NULL,
	"instrument_base_asset" VARCHAR NOT NULL,
	"instrument_quote_asset" VARCHAR NOT NULL,
    "instrument_exchanges_code" VARCHAR NOT NULL,
    "instrument_exchange_pair_code" VARCHAR NOT NULL,
	"instrument_pair_figi" VARCHAR,
	"instrument_figi" VARCHAR,
	"trade_start_timestamp" BIGINT NOT NULL,
	"trade_end_timestamp" BIGINT NOT NULL
);

-- FIGI (Financial Instrument Global Identifier) is a unique identifier for an instrument similar to ISIN

-- Instrument relations
-- instruments has exchanges (via exchange code)
-- instruments has assets (base and quote) via asset code

-- Download statistics i.e. when was the last download and how many assets, instruments etc.
-- Hash helps to compare if the data has changed.
CREATE TABLE mddb.stats(
    "stats_id" INTEGER NOT NULL PRIMARY KEY,
    "stats_download_timestamp" VARCHAR NOT NULL,
    "stats_hash" VARCHAR NOT NULL,
    "stats_assets" INTEGER NOT NULL,
    "stats_exchanges" INTEGER NOT NULL,
    "stats_instruments" INTEGER NOT NULL
);