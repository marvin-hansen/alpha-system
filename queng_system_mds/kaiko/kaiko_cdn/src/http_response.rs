/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use serde::{Deserialize, Serialize};

use worker::Response;

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponse {
    status: u16,
    message: String,
}

impl HttpResponse {
    /// Construct a new `HttpResponse` with the given status and message.
    pub const fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }

    /// Construct a successful `HttpResponse` (200) with the given message.
    pub fn success(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(200, message.to_string()))
    }

    /// Construct an not found error `HttpResponse` (404) with the given message.
    pub fn error_not_found(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(404, message.to_string()))
    }

    /// Construct a forbidden error `HttpResponse` (403) with the given message.
    pub fn error_forbidden(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(403, message.to_string()))
    }

    /// Construct an internal error `HttpResponse` (500) with the given message.
    pub fn error_internal(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(500, message.to_string()))
    }
}
