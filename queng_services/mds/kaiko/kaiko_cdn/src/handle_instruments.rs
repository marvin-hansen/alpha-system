use crate::fields::{AUTH_HEADER_KEY, INSTRUMENTS_KEY, METADATA_KV, RO_AUTH_KEY, RW_AUTH_KEY};
use crate::http_response::HttpResponse;
use common_metadata::MetaInstrumentsRoot;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

/// Handles the GET /instruments request by retrieving the instruments metadata
/// from the KV store and returning it as a JSON response.
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
pub async fn handle_get_instruments(
    req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
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

    // Get the metadata
    match kv
        .get(INSTRUMENTS_KEY)
        .json::<MetaInstrumentsRoot>()
        .await?
    {
        Some(instruments) => Response::from_json(&instruments),
        None => HttpResponse::error_not_found("Instruments not found!"),
    }
}

/// Handles the PUT /instruments request.
///
/// # Description
///
/// This function updates the instruments metadata by deserializing the request
/// body into a `MetaInstrumentsRoot` object, serializing it into a string, and storing
/// it in the KV storage under the key `INSTRUMENTS_KEY`.
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
pub async fn handle_put_instruments(
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
    let body = match req.json::<MetaInstrumentsRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaInstrumentsRoot from the body
    let updated_instruments = body;

    // Serialize the body into string
    let value = match to_string(&updated_instruments) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(INSTRUMENTS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}

/// Handles the POST /instruments request.
///
/// # Description
///
/// This function processes the POST request for instruments by deserializing the request
/// body into a `MetaInstrumentsRoot` object, serializing it into a JSON string, and storing
/// it in the KV storage under the key `INSTRUMENTS_KEY`.
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
pub async fn handle_post_instruments(
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

    // Create a new MetaInstrumentsRoot from the body
    let body = match req.json::<MetaInstrumentsRoot>().await {
        Ok(body) => body,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return HttpResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(INSTRUMENTS_KEY, value)?.execute().await {
        Ok(()) => HttpResponse::success("OK!"),
        Err(e) => HttpResponse::error_internal(&e.to_string()),
    }
}
