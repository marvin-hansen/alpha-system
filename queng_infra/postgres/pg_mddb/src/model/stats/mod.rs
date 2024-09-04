use bon::builder;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
pub struct PostgresStats {
    pub stats_id: i32,
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
pub struct CreatePostgresStats {
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
pub struct UpdatePostgresStats {
    pub stats_id: i32,
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}
