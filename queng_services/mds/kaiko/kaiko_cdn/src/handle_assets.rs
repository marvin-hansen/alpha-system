use crate::fields::{ASSETS_KEY, METADATA_KV};
use crate::handle_shared::GenericResponse;
use common_metadata::prelude::MetaAssetRoot;
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_assets(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv(METADATA_KV)?;

    match kv.get(ASSETS_KEY).json::<MetaAssetRoot>().await? {
        Some(assets) => {
            let res = match serde_json::to_vec(&assets) {
                Ok(res) => res,
                Err(e) => return GenericResponse::error_internal(&e.to_string()),
            };

            Response::from_json(&res)
        }
        None => GenericResponse::error_not_found("Assets not found!"),
    }
}

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
