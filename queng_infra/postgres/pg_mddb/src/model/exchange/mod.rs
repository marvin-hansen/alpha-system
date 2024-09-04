mod exchange_impl;
pub mod exchange_type_conversion;

use bon::builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
#[diesel(primary_key(exchange_id))]
pub struct Exchange {
    pub exchange_id: String,
    pub exchange_name: String,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
#[diesel(primary_key(exchange_id))]
pub struct CreateExchange {
    pub exchange_id: String,
    pub exchange_name: String,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
pub struct UpdateExchange {
    pub exchange_name: String,
}
