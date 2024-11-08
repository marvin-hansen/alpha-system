use crate::fields::{METADATA_KV, STATS_KEY};
use crate::handle_shared::GenericResponse;
use common_metadata::prelude::MetaStats;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_stats(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    match kv.get(STATS_KEY).json::<MetaStats>().await? {
        Some(stats) => Response::from_json(&stats),
        None => GenericResponse::error_not_found("Stats not found!"),
    }
}

pub async fn handle_put_stats(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaStats>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Serialize the body into string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(STATS_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
    }
}

pub async fn handle_post_stats(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    // Create a new MetaStats from the body
    let body = match req.json::<MetaStats>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(STATS_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
    }
}
