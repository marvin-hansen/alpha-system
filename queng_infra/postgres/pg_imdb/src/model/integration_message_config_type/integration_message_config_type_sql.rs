use crate::model::integration_message_config_type::MessageConfig;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Integer, Record, Text};
use diesel::{deserialize, serialize};

impl ToSql<crate::schema::imdb::sql_types::IntegrationMessageConfig, Pg> for MessageConfig {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        serialize::WriteTuple::<(Integer, Text, Integer, Integer)>::write_tuple(
            &(
                self.id.to_owned(),
                self.name.clone(),
                self.version.to_owned(),
                self.exchange_id.to_owned(),
            ),
            &mut out.reborrow(),
        )
    }
}

impl FromSql<crate::schema::imdb::sql_types::IntegrationMessageConfig, Pg> for MessageConfig {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (id, name, version, exchange_id) =
            FromSql::<Record<(Integer, Text, Integer, Integer)>, Pg>::from_sql(bytes)?;

        Ok(Self {
            id,
            name,
            version,
            exchange_id,
        })
    }
}
