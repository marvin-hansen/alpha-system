use crate::model::protocol_type::{PgProtocolType, ProtocolType};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize};
use std::io::Write;

impl ToSql<PgProtocolType, Pg> for ProtocolType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ProtocolType::UnknownProtocol => out.write_all(b"UnknownProtocol")?,
            ProtocolType::GRPC => out.write_all(b"GRPC")?,
            ProtocolType::HTTP => out.write_all(b"HTTP")?,
            ProtocolType::UDP => out.write_all(b"UDP")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PgProtocolType, Pg> for ProtocolType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"UnknownProtocol" => Ok(ProtocolType::UnknownProtocol),
            b"GRPC" => Ok(ProtocolType::GRPC),
            b"HTTP" => Ok(ProtocolType::HTTP),
            b"UDP" => Ok(ProtocolType::UDP),
            _ => Err(DatabaseError(
                DatabaseErrorKind::SerializationFailure,
                Box::new(format!(
                    "Unrecognized ProtocolType Enum variant: {:?}",
                    String::from_utf8_lossy(bytes.as_bytes())
                )),
            )
            .into()),
        }
    }
}
