use crate::store::DB;
use crate::types::health::Health;
use warp;

pub(crate) async fn get_health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}

pub(crate) async fn get_assets_handler(store: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = store.read().await;
    let result = lock.assets();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_exchanges_handler(store: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = store.read().await;
    let result = lock.exchanges();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_instruments_handler(
    store: DB,
) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = store.read().await;
    let result = lock.instruments();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_symbol_mapping_handler(
    store: DB,
) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = store.read().await;
    let result = lock.symbol_mapping();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_stats_handler(store: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = store.read().await;
    let result = lock.stats();
    Ok(warp::reply::json(result))
}
