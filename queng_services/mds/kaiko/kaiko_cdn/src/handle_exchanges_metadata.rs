use crate::GenericResponse;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_exchanges(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let kv = ctx.kv("METADATA")?;

    Response::from_json(&GenericResponse {
        status: 200,
        message: "exchanges!".to_string(),
    })
}
