// In Postgres, a namespace is called a schema and partitions tables into logical groups.
// Note, a query against any other scheme than the default "public" schema requires
// the schema prefix before the table. Technically, the public prefix is optional
// in Postgres so you can write queries with or without it.

// For the public table portfolio you can write SELECT * FROM portfolio;
// For the system table service you have to write SELECT * FROM system.service;

// Note, any change on these constants requires the re-creation of the affected
// DB, schema, or table(s) before changes become effective.

/// The main database is called "specs" because it contains all sorts of specifications.
pub(crate) const DB_NAME: &str = "specs";

/// The system schema contains all the internally used tables of the system.
pub(crate) const SYSTEM_SCHEMA: &str = "system";

/// The service table contains a configuration for each microservice of the system.
pub(crate) const SERVICE_TABLE: &str = "service";

/// Primary index of the service table
pub(crate) const SERVICE_TABLE_INDEX: &str = "idx_service_id";

/// The default schema, which is called public in Postgres,
/// contains all the domain specific tables the system needs to operate.
pub(crate) const DEFAULT_SCHEMA: &str = "public";

/// The portfolio table contains all the portfolio configurations of the system.
pub(crate) const PORTFOLIO_TABLE: &str = "portfolio";

/// The primary index of the portfolio table
pub(crate) const PORTFOLIO_TABLE_INDEX: &str = "idx_portfolio_id";

/// The instrument table contains all the instrument configurations
/// that are used in the portfolio table.
pub(crate) const INSTRUMENT_TABLE: &str = "instrument";
pub(crate) const PORTFOLIO_INSTRUMENT_TABLE: &str = "portfolio_instrument";
pub(crate) const PORTFOLIO_INSTRUMENT_TABLE_INDEX: &str = "idx_instrument_id";
