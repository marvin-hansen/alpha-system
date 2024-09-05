mod stat_type_conversion;

use diesel::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stat {
    pub stats_id: i32,
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateStat {
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::stats , primary_key(stats_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateStat {
    pub stats_id: i32,
    pub stats_hash: String,
    pub stats_download_timestamp: String,
    pub stats_number_assets: i32,
    pub stats_number_exchanges: i32,
    pub stats_number_instruments: i32,
}
