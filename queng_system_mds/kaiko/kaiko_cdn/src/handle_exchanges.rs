use crate::fields::{AUTH_HEADER_KEY, EXCHANGES_KEY, METADATA_KV, RO_AUTH_KEY, RW_AUTH_KEY};

use crate::http_response::HttpResponse;
use common_metadata::MetaExchangesRoot;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

/// Handles the GET /exchanges request by retrieving the exchanges metadata from the KV
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
pub async fn handle_get_exchanges(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    // Check authentication
    let auth_header = match req.headers().get(AUTH_HEADER_KEY)? {
        Some(header) => header,
        None => return HttpResponse::error_forbidden("access denied"),
    };

    if auth_header != RO_AUTH_KEY {
        return HttpResponse::error_forbidden("access denied");
    }

    // Get KV store
    let kv = match ctx.kv(METADATA_KV) {
        Ok(kv) => kv,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    match kv.get(EXCHANGES_KEY).json::<MetaExchangesRoot>().await? {
        Some(exchanges) => Response::from_json(&exchanges),
        None => HttpResponse::error_not_found("Exchanges not found!"),
    }
}

/// Updates the exchanges metadata by storing the provided `MetaExchangesRoot`
/// in the KV storage under the `EXCHANGES_KEY`.
///
/// # Arguments
///
/// * `req` - A mutable `Request` containing the JSON body to be updated.
/// * `ctx` - A `RouteContext` providing context for the route, including access to KV storage.
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation.
///
/// # Errors
///
/// Returns a generic internal error response if there is an issue with deserializing
/// the request body, serializing the updated data, or updating the KV storage.
///
pub async fn handle_put_exchanges(
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

    // Get KV store
    let kv = match ctx.kv(METADATA_KV) {
        Ok(kv) => kv,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Get the body of the request
    let body = match req.json::<MetaExchangesRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaExchangesRoot from the body
    let new_exchanges = body;

    // Serialize the body into string
    let value = match to_string(&new_exchanges) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(EXCHANGES_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}

/// Handle POST /exchanges request
///
/// # Description
///
/// This function handles the POST /exchanges request by deserializing the request body
/// into a `MetaExchangesRoot` object, serializing it into a string, and storing it in the KV
/// storage under the key `EXCHANGES_KEY`.
///
/// # Arguments
///
/// * `req` - The incoming request
/// * `ctx` - The route context
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation
///
/// # Errors
///
/// Returns a generic internal error response if there is an issue with deserializing the request
/// body, serializing the updated data, or storing the data in the KV storage.
///
pub async fn handle_post_exchanges(
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

    // Get KV store
    let kv = match ctx.kv(METADATA_KV) {
        Ok(kv) => kv,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Get the body of the request
    let body = match req.json::<MetaExchangesRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaExchangesRoot from the body
    let new_exchanges = body;

    // Serialize the body into string
    let value = match to_string(&new_exchanges) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(EXCHANGES_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}
