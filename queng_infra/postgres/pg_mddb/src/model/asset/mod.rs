mod asset_impl;

use bon::builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
pub struct PostgresAsset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
pub struct CreatePostgresAsset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets)]
pub struct UpdatePostgresAsset {
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}
