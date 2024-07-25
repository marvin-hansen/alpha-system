use crate::common::all_db_constants::{
    DEFAULT_SCHEMA, INSTRUMENT_TABLE, PORTFOLIO_INSTRUMENT_TABLE_INDEX,
};
use crate::db::Specs;

// ...Many to Many Relationships When Designing A Database.
// https://medium.com/@emekadc/how-to-implement-one-to-one-one-to-many-and-many-to-many-relationships-when-designing-a-database-9da2de684710

// PostgreSQL CREATE TABLE
// https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-create-table/

// PostgreSQL CREATE INDEX
// https://www.postgresqltutorial.com/postgresql-indexes/postgresql-create-index/

impl Specs {
    pub(crate) fn generate_instrument_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {DEFAULT_SCHEMA}.{INSTRUMENT_TABLE} (
            code                            VARCHAR PRIMARY KEY,
            class                           VARCHAR NOT NULL,
            exchange_code                   VARCHAR NOT NULL,
            exchange_pair_code              VARCHAR NOT NULL,
            base_asset                      VARCHAR NOT NULL,
            quote_asset                     VARCHAR NOT NULL,
            instrument_figi                 VARCHAR
            );
            "
        )
    }

    pub(crate) fn generate_instrument_table_indexes_ddl(&self) -> String {
        format!(
            "
            CREATE INDEX {PORTFOLIO_INSTRUMENT_TABLE_INDEX} ON {DEFAULT_SCHEMA}.{INSTRUMENT_TABLE}(code);
            "
        )
    }
}
