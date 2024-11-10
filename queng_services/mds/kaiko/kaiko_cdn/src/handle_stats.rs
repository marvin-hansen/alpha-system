use crate::fields::{AUTH_HEADER_KEY, METADATA_KV, RO_AUTH_KEY, RW_AUTH_KEY, STATS_KEY};
use crate::http_response::HttpResponse;
use common_metadata::prelude::MetaStats;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

/// Handles the GET /stats request by retrieving the stats metadata from the KV
/// store and returning it as a JSON response.
///
/// # Arguments
///
/// * `_request` - The incoming request
/// * `ctx` - The route context
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation
///
pub async fn handle_get_stats(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    // Check authentication
    let auth_header = match req.headers().get(AUTH_HEADER_KEY)? {
        Some(header) => header,
        None => return HttpResponse::error_forbidden("access denied"),
    };

    if auth_header != RO_AUTH_KEY {
        return HttpResponse::error_forbidden("access denied");
    }

    // Get the KV store
    let kv = ctx.kv(METADATA_KV)?;

    // Get the metadata
    match kv.get(STATS_KEY).json::<MetaStats>().await? {
        Some(stats) => Response::from_json(&stats),
        None => HttpResponse::error_not_found("Stats not found!"),
    }
}

/// Handles the PUT /stats request by deserializing the request body into a `MetaStats`
/// object, serializing it into a JSON string, and storing it in the KV storage under the
/// key `STATS_KEY`.
///
/// # Arguments
///
/// * `req` - A mutable `Request` object containing the JSON body to be stored.
/// * `ctx` - A `RouteContext` providing context for the route, including access to KV storage.
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation
///
pub async fn handle_put_stats(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    // Check authentication
    let auth_header = match req.headers().get(AUTH_HEADER_KEY)? {
        Some(header) => header,
        None => return HttpResponse::error_forbidden("access denied"),
    };

    if auth_header != RW_AUTH_KEY {
        return HttpResponse::error_forbidden("access denied");
    }

    // Get the KV store
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaStats>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Serialize the body into json string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(STATS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}

/// Handles the POST /stats request by deserializing the request body into a `MetaStats`
/// object, serializing it into a JSON string, and storing it in the KV storage under the
/// key `STATS_KEY`.
///
/// # Arguments
///
/// * `req` - A mutable `Request` object containing the JSON body to be stored.
/// * `ctx` - A `RouteContext` providing context for the route, including access to KV storage.
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation.
///
/// # Errors
///
/// Returns a generic internal error response if there is an issue with deserializing the request
/// body, serializing the data, or storing it in the KV storage.
///
pub async fn handle_post_stats(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    // Check authentication
    let auth_header = match req.headers().get(AUTH_HEADER_KEY)? {
        Some(header) => header,
        None => return HttpResponse::error_forbidden("access denied"),
    };

    if auth_header != RW_AUTH_KEY {
        return HttpResponse::error_forbidden("access denied");
    }

    // Get the KV store
    let kv = ctx.kv(METADATA_KV)?;

    // Create a new MetaStats from the body
    let body = match req.json::<MetaStats>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(STATS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}
