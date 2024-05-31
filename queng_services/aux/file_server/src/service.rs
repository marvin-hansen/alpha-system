use crate::store::Store;
use warp;

pub(crate) async fn get_assets_from_store(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.assets();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_exchanges_from_store(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.exchanges();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_instruments_from_store(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.instruments();
    Ok(warp::reply::json(result))
}

pub(crate) async fn get_symbol_mapping_from_store(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.symbol_mapping();
    Ok(warp::reply::json(result))
}
