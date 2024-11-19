use crate::MetaDataStore;

pub async fn get_assets_handler(store: MetaDataStore) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.read().await;
    let result = guard.assets();
    Ok(warp::reply::json(result))
}

pub async fn get_exchanges_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.read().await;
    let result = guard.exchanges();
    Ok(warp::reply::json(result))
}

pub async fn get_instruments_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.read().await;
    let result = guard.instruments();
    Ok(warp::reply::json(result))
}

pub async fn get_stats_handler(store: MetaDataStore) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.read().await;
    let result = guard.stats();
    Ok(warp::reply::json(result))
}

// ###############################################################################
// Health handler
// ###############################################################################
pub async fn get_health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health<'s> {
    status: &'s str,
}

impl Health<'_> {
    pub const fn ok() -> Self {
        Self { status: "OK" }
    }
}
