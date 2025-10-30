// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, HttpResponseBuilder};
use common_library::error::ErrToResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("An internal server error occurred.")]
    InternalError(String),
}

impl ErrToResponse for PluginError {
    fn error(&self) -> &'static str {
        "PluginError"
    }

    fn err_type(&self) -> &'static str {
        match self {
            Self::InternalError(_) => stringify!(InternalError),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> HttpResponseBuilder {
        match self {
            Self::InternalError(_) => HttpResponse::InternalServerError(),
        }
    }

    fn handle(&self) {
        let Self::InternalError(err) = self;
        log::error!("An internal server error occurred during authentication: {err}");
    }
}
