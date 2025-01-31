/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::http_response::HttpResponse;
use worker::{Request, Response, RouteContext};

/// Handles the GET /health request by returning a successful response.
///
/// # Arguments
///
/// * `_req` - The incoming request
/// * `_ctx` - The route context
///
/// # Returns
///
/// * `worker::Result<Response>` - A response indicating success or failure of the operation
///
pub async fn handle_get_health(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    HttpResponse::success("ok!")
}
