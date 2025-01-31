/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod exchange_impl;
pub mod exchange_type_conversion;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[derive(
    Debug, Clone, PartialEq, Eq, Queryable, Selectable, Identifiable, Insertable, AsChangeset,
)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
#[diesel(primary_key(exchange_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exchange {
    pub exchange_id: String,
    pub exchange_name: String,
    pub exchange_hash: String,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
#[diesel(primary_key(exchange_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateExchange {
    pub exchange_id: String,
    pub exchange_name: String,
    pub exchange_hash: String,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::exchanges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateExchange {
    pub exchange_name: String,
    pub exchange_hash: String,
}
