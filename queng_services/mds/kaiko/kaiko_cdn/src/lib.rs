use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct HealthResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/health", handle_get_health)
        .run(req, env)
        .await
}

pub async fn handle_get_health(_: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_json(&HealthResponse {
        status: 200,
        message: "ok!".to_string(),
    })
}
