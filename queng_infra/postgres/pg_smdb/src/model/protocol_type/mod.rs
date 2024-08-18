use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::sql_types::SqlType;

mod protocol_type_conversion;
mod protocol_type_sql;

//  Diesel type mapping requires a struct to derive a SqlType for custom ToSql and FromSql implementations
#[derive(SqlType)]
#[diesel(sql_type = protocol_type)]
#[diesel(postgres_type(name = "protocol_type", schema = "smdb"))]
pub struct PgProtocolType;

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type = PgProtocolType)]
#[allow(clippy::upper_case_acronyms)]
pub enum ProtocolType {
    UnknownProtocol,
    GRPC,
    HTTP,
    UDP,
}
