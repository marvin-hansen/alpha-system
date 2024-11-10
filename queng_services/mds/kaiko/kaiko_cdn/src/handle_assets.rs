use crate::fields::{ASSETS_KEY, AUTH_HEADER_KEY, METADATA_KV, RO_AUTH_KEY, RW_AUTH_KEY};
use crate::handle_shared::HttpResponse;
use common_metadata::prelude::MetaAssetRoot;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

/// Handles the GET /assets request by retrieving the assets metadata from the KV
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
pub async fn handle_get_assets(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    // Check authentication
    // https://developers.cloudflare.com/workers/examples/auth-with-headers/
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
    //  https://developers.cloudflare.com/kv/get-started/#5-access-your-kv-namespace-from-your-worker
    match kv.get(ASSETS_KEY).json::<MetaAssetRoot>().await? {
        Some(assets) => Response::from_json(&assets),
        None => HttpResponse::error_not_found("Assets not found!"),
    }
}

/// Handles the PUT /assets request.
///
/// # Description
///
/// This function processes the PUT request for assets by deserializing the request
/// body into a `MetaAssetRoot` object, serializing it into a JSON string, and storing
/// it in the KV storage under the key `ASSETS_KEY`.
///
/// # Arguments
///
/// * `req` - A mutable `Request` object containing the JSON body to be updated.
/// * `ctx` - A `RouteContext` providing context for the route, including access to KV storage.
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation.
///
/// # Errors
///
/// Returns a generic internal error response if there is an issue with deserializing the request
/// body, serializing the updated data, or updating the KV storage.
///
pub async fn handle_put_assets(
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
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaAssetRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaAssetRoot from the body
    let new_assets = MetaAssetRoot {
        result: body.result,
        data: body.data,
    };

    // Serialize the body into string
    let value = match to_string(&new_assets) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(ASSETS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}

/// Handles the POST /assets request.
///
/// # Description
///
/// This function processes the POST request for assets by deserializing the request
/// body into a `MetaAssetRoot` object, serializing it into a JSON string, and storing
/// it in the KV storage under the key `ASSETS_KEY`.
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
pub async fn handle_post_assets(
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
    let kv = ctx.kv(METADATA_KV)?;

    // Create a new MetaAssetRoot from the body
    let body = match req.json::<MetaAssetRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(ASSETS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}
