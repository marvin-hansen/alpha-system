/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

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
            Self::UnknownProtocol => out.write_all(b"UnknownProtocol")?,
            Self::GRPC => out.write_all(b"GRPC")?,
            Self::HTTP => out.write_all(b"HTTP")?,
            Self::UDP => out.write_all(b"UDP")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PgProtocolType, Pg> for ProtocolType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"UnknownProtocol" => Ok(Self::UnknownProtocol),
            b"GRPC" => Ok(Self::GRPC),
            b"HTTP" => Ok(Self::HTTP),
            b"UDP" => Ok(Self::UDP),
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
