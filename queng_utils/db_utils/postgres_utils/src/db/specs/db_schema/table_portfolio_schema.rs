use crate::common::all_db_constants::{DEFAULT_SCHEMA, PORTFOLIO_TABLE, PORTFOLIO_TABLE_INDEX};
use crate::db::Specs;

// Composite Types
//  https://www.postgresql.org/docs/current/rowtypes.html#ROWTYPES

// PostgreSQL CREATE TABLE
// https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-create-table/

// PostgreSQL CREATE INDEX
// https://www.postgresqltutorial.com/postgresql-indexes/postgresql-create-index/

// ...Many to Many Relationships When Designing A Database.
// https://medium.com/@emekadc/how-to-implement-one-to-one-one-to-many-and-many-to-many-relationships-when-designing-a-database-9da2de684710

impl Specs {
    pub(crate) fn generate_portfolio_table_account_type_ddl(&self) -> String {
        r#"
        CREATE TYPE "account_type" AS ENUM(
            'NullVal',
            'Spot',
            'Margin'
            'Future'
        );
        "#
        .to_string()
    }

    pub(crate) fn generate_portfolio_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {DEFAULT_SCHEMA}.{PORTFOLIO_TABLE} (
            portfolio_id                    INT PRIMARY KEY,
            portfolio_description           VARCHAR NOT NULL,
            portfolio_account_type          account_type NOT NULL,
            portfolio_account_id            VARCHAR UNIQUE NOT NULL,
            portfolio_currency              VARCHAR NOT NULL,
            portfolio_cash                  DOUBLE PRECISION NOT NULL,
            portfolio_margin                DOUBLE PRECISION NOT NULL,
            portfolio_max_drawdown          DOUBLE PRECISION NOT NULL,
            instrument_max_allocation       DOUBLE PRECISION NOT NULL,
            instrument_max_drawdown         DOUBLE PRECISION NOT NULL,
            portfolio_free_margin           DOUBLE PRECISION NOT NULL,
            portfolio_free_cash             DOUBLE PRECISION NOT NULL,
            portfolio_free_margin_percent   DOUBLE PRECISION NOT NULL,
            portfolio_free_cash_percent     DOUBLE PRECISION NOT NULL
            );
            "
        )
    }

    pub(crate) fn generate_portfolio_table_index_ddl(&self) -> String {
        format!(
            "
             CREATE INDEX {PORTFOLIO_TABLE_INDEX} ON {DEFAULT_SCHEMA}.{PORTFOLIO_TABLE}(portfolio_id);
            "
        )
    }
}
