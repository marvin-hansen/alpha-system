pub const SCHEMA_UP: &'static str = r#"
-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS cmdb;

CREATE TABLE cmdb.instrument(
	"code" VARCHAR NOT NULL PRIMARY KEY,
	"class" VARCHAR NOT NULL,
	"exchange_code" VARCHAR NOT NULL,
	"exchange_pair_code" VARCHAR NOT NULL,
	"base_asset" VARCHAR NOT NULL,
	"quote_asset" VARCHAR NOT NULL,
	"instrument_figi" VARCHAR
);

CREATE TABLE cmdb.portfolio(
	"portfolio_id" INTEGER NOT NULL PRIMARY KEY,
	"portfolio_description" VARCHAR NOT NULL,
	"portfolio_account_type" INTEGER NOT NULL,
	"portfolio_account_id" VARCHAR NOT NULL,
	"portfolio_currency" VARCHAR NOT NULL,
	"portfolio_cash" FLOAT8 NOT NULL,
	"portfolio_margin" FLOAT8 NOT NULL,
	"portfolio_max_drawdown" FLOAT8 NOT NULL,
	"instrument_max_allocation" FLOAT8 NOT NULL,
	"instrument_max_drawdown" FLOAT8 NOT NULL,
	"portfolio_free_margin" FLOAT8 NOT NULL,
	"portfolio_free_cash" FLOAT8 NOT NULL,
	"portfolio_free_margin_percent" FLOAT8 NOT NULL,
	"portfolio_free_cash_percent" FLOAT8 NOT NULL
);

CREATE TABLE cmdb.portfolio_instrument (
	portfolio_id INTEGER REFERENCES cmdb.portfolio(portfolio_id),
	instrument_id VARCHAR REFERENCES cmdb.instrument(code),
	PRIMARY KEY(portfolio_id, instrument_id)
);
"#;
