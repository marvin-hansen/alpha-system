mod exchange_type_conversion;

use bon::builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges, primary_key(exchanges_code))]
pub struct PostgresExchange {
    pub exchanges_code: String,
    pub exchanges_name: String,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges, primary_key(exchanges_code))]
pub struct CreatePostgresExchange {
    pub exchanges_code: String,
    pub exchanges_name: String,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
pub struct UpdatePostgresExchange {
    pub exchanges_name: String,
}
