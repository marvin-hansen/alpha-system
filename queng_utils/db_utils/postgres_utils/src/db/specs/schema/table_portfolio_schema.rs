use crate::db::all_db_constants::{DEFAULT_SCHEMA, PORTFOLIO_TABLE};
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
    pub(crate) fn generate_portfolio_table_types_ddl(&self) -> String {
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
            portfolio_free_cash_percent     DOUBLE PRECISION NOT NULL
            );

            CREATE INDEX idx_portfolio_id ON {DEFAULT_SCHEMA}.{PORTFOLIO_TABLE}(portfolio_id);
            CREATE INDEX idx_portfolio_account_id ON {DEFAULT_SCHEMA}.{PORTFOLIO_TABLE}(account_id);
            "
        )
    }
}
