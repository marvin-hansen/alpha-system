use crate::{TestAsset, ASSETS_KEY};
use serde_json::to_string;
use worker::{Request, Response, RouteContext};

pub async fn handle_put_assets(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv("METADATA")?;

    // Get the body of the request - Note that AnimalRescue implements Deserialize
    let body = match req.json::<TestAsset>().await {
        Ok(body) => body,
        Err(e) => return Response::error(e.to_string(), 500),
    };

    let new_assets = TestAsset { name: body.name };

    // Serialize the body to a string
    let value = match to_string(&new_assets) {
        Ok(value) => value,
        Err(e) => return Response::error(e.to_string(), 500),
    };

    // Store the new value in KV
    match kv.put(ASSETS_KEY, value)?.execute().await {
        Ok(()) => Response::from_json(&new_assets),
        Err(e) => Response::error(e.to_string(), 500),
    }
}

pub async fn handle_post_assets(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv("METADATA")?;

    // Get the body of the request - Note that AnimalRescue implements Deserialize
    let body = match req.json::<TestAsset>().await {
        Ok(body) => body,
        Err(e) => return Response::error(e.to_string(), 500),
    };

    // Serialize the body to a string
    let value = match to_string(&body) {
        Ok(value) => value,
        Err(e) => return Response::error(e.to_string(), 500),
    };

    // Store the value in KV
    kv.put(ASSETS_KEY, value)?.execute().await?;

    // Return the response
    Response::from_json(&body)
}

pub async fn handle_get_assets(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv("METADATA")?;

    match kv.get(ASSETS_KEY).json::<TestAsset>().await? {
        Some(assets) => Response::from_json(&assets),
        None => Response::error("Assets not found", 404),
    }
}
