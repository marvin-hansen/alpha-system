use crate::GenericResponse;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_health(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "OK!".to_string(),
    })
}
