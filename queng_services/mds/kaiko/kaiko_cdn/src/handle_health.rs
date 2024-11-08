use crate::handle_shared::GenericResponse;
use worker::{Request, Response, RouteContext};

pub async fn handle_get_health(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    GenericResponse::success("ok!")
}
