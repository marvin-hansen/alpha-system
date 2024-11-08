use crate::GenericResponse;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_instruments(
    _: Request,
    _ctx: RouteContext<()>,
) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "instruments!".to_string(),
    })
}
