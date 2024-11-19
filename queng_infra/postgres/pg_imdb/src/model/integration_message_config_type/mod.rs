use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;

mod integration_message_config_type_conversion;
mod integration_message_config_type_sql;

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type=crate::schema::imdb::sql_types::IntegrationMessageConfig)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageConfig {
    id: i32,
    name: String,
    version: i32,
    exchange_id: i32,
}

impl MessageConfig {
    #[must_use]
    pub const fn new(id: i32, name: String, version: i32, exchange_id: i32) -> Self {
        Self {
            id,
            name,
            version,
            exchange_id,
        }
    }
}
