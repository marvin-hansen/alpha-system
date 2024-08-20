use crate::types::health::Health;
use crate::types::MetaDataStore;
use autometrics::autometrics;

pub(crate) async fn get_metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match autometrics::prometheus_exporter::encode_to_string() {
        Ok(metrics) => Ok(warp::reply::json(&metrics)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[autometrics]
pub(crate) async fn get_health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}

#[autometrics]
pub(crate) async fn get_assets_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.load();
    let result = guard.assets();
    Ok(warp::reply::json(result))
}

#[autometrics]
pub(crate) async fn get_exchanges_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.load();
    let result = guard.exchanges();
    Ok(warp::reply::json(result))
}

#[autometrics]
pub(crate) async fn get_instruments_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.load();
    let result = guard.instruments();
    Ok(warp::reply::json(result))
}

#[autometrics]
pub(crate) async fn get_stats_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.load();
    let result = guard.stats();
    Ok(warp::reply::json(result))
}
