use crate::db::all_db_constants::{
    DEFAULT_SCHEMA, INSTRUMENT_TABLE, PORTFOLIO_INSTRUMENT_TABLE, PORTFOLIO_TABLE,
};
use crate::db::Specs;

// ...Many to Many Relationships When Designing A Database.
// https://medium.com/@emekadc/how-to-implement-one-to-one-one-to-many-and-many-to-many-relationships-when-designing-a-database-9da2de684710

impl Specs {
    pub(crate) fn generate_portfolio_instrument_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS {DEFAULT_SCHEMA}.{PORTFOLIO_INSTRUMENT_TABLE} (
            portfolio_id                    INT NOT NULL,
            instrument_id                   INT NOT NULL,
            PRIMARY KEY(portfolio_id, instrument_id),
            FOREIGN KEY(portfolio_id) REFERENCES {DEFAULT_SCHEMA}.{PORTFOLIO_TABLE}(portfolio_id),
            FOREIGN KEY(instrument_id) REFERENCES {DEFAULT_SCHEMA}.{INSTRUMENT_TABLE}(id)
            );
            "
        )
    }
}
