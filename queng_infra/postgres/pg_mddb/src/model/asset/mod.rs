mod asset_impl;
pub mod asset_type_conversion;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName, Selectable};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Queryable,
    QueryableByName,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Asset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_class: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
    pub asset_hash: String,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAsset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_class: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
    pub asset_hash: String,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateAsset {
    pub asset_name: String,
    pub asset_class: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
    pub asset_hash: String,
}
