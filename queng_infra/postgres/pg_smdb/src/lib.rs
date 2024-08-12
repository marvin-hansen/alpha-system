pub mod model;
pub(crate) mod schema;

pub type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;
