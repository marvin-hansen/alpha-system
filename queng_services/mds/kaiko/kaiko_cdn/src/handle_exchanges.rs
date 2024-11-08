use crate::fields::{EXCHANGES_KEY, METADATA_KV};
use crate::handle_shared::GenericResponse;
use common_metadata::prelude::MetaExchangesRoot;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_exchanges(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    match kv.get(EXCHANGES_KEY).json::<MetaExchangesRoot>().await? {
        Some(exchanges) => {
            let res = match serde_json::to_vec(&exchanges) {
                Ok(res) => res,
                Err(e) => return GenericResponse::error_internal(&e.to_string()),
            };

            Response::from_json(&res)
        }
        None => GenericResponse::error_not_found("Exchanges not found!"),
    }
}

pub async fn handle_put_exchanges(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaExchangesRoot>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaExchangesRoot from the body
    let new_exchanges = MetaExchangesRoot {
        result: body.result,
        data: body.data,
    };

    // Serialize the body into string
    let value = match to_string(&new_exchanges) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Update the new value in KV
    match kv.put(EXCHANGES_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
    }
}

pub async fn handle_post_exchanges(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    // Get the body of the request
    let body = match req.json::<MetaExchangesRoot>().await {
        Ok(body) => body,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Create a new MetaExchangesRoot from the body
    let new_exchanges = MetaExchangesRoot {
        result: body.result,
        data: body.data,
    };

    // Serialize the body into string
    let value = match to_string(&new_exchanges) {
        Ok(value) => value,
        Err(e) => return GenericResponse::error_internal(&e.to_string()),
    };

    // Store the value in KV
    match kv.put(EXCHANGES_KEY, value)?.execute().await {
        Ok(()) => GenericResponse::success("OK!"),
        Err(e) => GenericResponse::error_internal(&e.to_string()),
    }
}
