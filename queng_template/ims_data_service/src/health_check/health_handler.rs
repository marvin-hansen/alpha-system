/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

pub(crate) async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = { String::from("Online") };
    Ok(warp::reply::json(&result))
}
