use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;

// Diesel already generated a struct for custom ToSql and FromSql implementations.
use crate::schema::smdb::sql_types::ServiceEndpoint as PgServiceEndpoint;

mod endpoint_type_impl;

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type = PgServiceEndpoint)]
pub struct Endpoint {
    pub name: String,
    pub version: i32,
    pub base_uri: String,
    pub port: i32,
    pub protocol: i32,
}

impl Endpoint {
    pub fn new(name: String, version: i32, base_uri: String, port: i32, protocol: i32) -> Self {
        Self {
            name,
            version,
            base_uri,
            port,
            protocol,
        }
    }
}
