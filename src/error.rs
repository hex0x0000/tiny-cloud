// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, HttpResponseBuilder};
use common_library::error::ErrToResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid JSON request: {0}")]
    Json(String),
    #[error("Invalid URL query: {0}")]
    Query(String),
    #[error("Invalid Multipart request: {0}")]
    Multipart(String),
}

impl ErrToResponse for RequestError {
    fn error(&self) -> &'static str {
        "RequestError"
    }

    fn err_type(&self) -> &'static str {
        match &self {
            Self::Json(_) => stringify!(Json),
            Self::Query(_) => stringify!(Query),
            Self::Multipart(_) => stringify!(Multipart),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> HttpResponseBuilder {
        HttpResponse::BadRequest()
    }

    fn handle(&self) {
        log::debug!("Invalid request from client: {}", self);
    }
}
