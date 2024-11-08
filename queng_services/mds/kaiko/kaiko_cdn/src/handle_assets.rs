use crate::fields::{ASSETS_KEY, METADATA_KV};
use crate::handle_shared::GenericResponse;
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
pub async fn handle_get_assets(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    match kv.get(ASSETS_KEY).json::<MetaAssetRoot>().await? {
        Some(assets) => Response::from_json(&assets),
        None => GenericResponse::error_not_found("Assets not found!"),
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
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaAssetRoot>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaAssetRoot from the body
    let new_assets = MetaAssetRoot {
        result: body.result,
        data: body.data,
    };

    // Serialize the body into string
    let value = match to_string(&new_assets) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(ASSETS_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
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
    let kv = ctx.kv(METADATA_KV)?;

    // Create a new MetaAssetRoot from the body
    let body = match req.json::<MetaAssetRoot>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(ASSETS_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
    }
}
