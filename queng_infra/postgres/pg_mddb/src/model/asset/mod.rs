mod asset_impl;
pub mod asset_type_conversion;

use bon::builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
pub struct Asset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets, primary_key(asset_code))]
pub struct CreateAsset {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::assets)]
pub struct UpdateAsset {
    pub asset_name: String,
    pub asset_classes: Vec<Option<String>>,
    pub asset_figi: Option<String>,
}
