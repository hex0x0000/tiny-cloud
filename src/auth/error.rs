// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, HttpResponseBuilder};
use common_library::error::ErrToResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("An internal server error occurred")]
    InternalError(String),
    #[error("Bad credentials were given: {0}")]
    BadCredentials(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid registration credentials")]
    InvalidRegCredentials,
    #[error("Invalid session")]
    InvalidSession,
    #[error("Invalid TOTP token")]
    InvalidTOTP,
}

impl ErrToResponse for AuthError {
    fn error(&self) -> &'static str {
        "AuthError"
    }

    fn err_type(&self) -> &'static str {
        match self {
            Self::BadCredentials(_) => stringify!(BadCredentials),
            Self::InvalidCredentials => stringify!(InvalidCredentials),
            Self::InvalidRegCredentials => stringify!(InvalidRegCredentials),
            Self::InvalidSession => stringify!(InvalidSession),
            Self::InternalError(_) => stringify!(InternalError),
            Self::InvalidTOTP => stringify!(InvalidTOTP),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> HttpResponseBuilder {
        match self {
            Self::BadCredentials(_) => HttpResponse::BadRequest(),
            Self::InvalidCredentials => HttpResponse::Unauthorized(),
            Self::InvalidRegCredentials => HttpResponse::Unauthorized(),
            Self::InvalidSession => HttpResponse::Unauthorized(),
            Self::InvalidTOTP => HttpResponse::Unauthorized(),
            Self::InternalError(_) => HttpResponse::InternalServerError(),
        }
    }

    fn handle(&self) {
        if let Self::InternalError(err) = self {
            log::error!("An internal server error occurred during authentication: {err}");
        }
    }
}
