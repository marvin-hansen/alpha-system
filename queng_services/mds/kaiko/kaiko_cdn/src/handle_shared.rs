use serde::{Deserialize, Serialize};

use worker::Response;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct GenericResponse {
    status: u16,
    message: String,
}

impl GenericResponse {
    /// Construct a new `GenericResponse` with the given status and message.
    ///
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }

    pub fn success(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(200, message.to_string()))
    }

    pub fn error_not_found(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(404, message.to_string()))
    }

    pub fn error_internal(message: &str) -> worker::Result<Response> {
        Response::from_json(&Self::new(500, message.to_string()))
    }
}
