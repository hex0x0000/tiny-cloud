// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, HttpResponseBuilder};
use common_library::error::ErrToResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("An internal server error occurred")]
    InternalError(String),
    #[error("Token was not found")]
    NotFound,
    #[error("Token expired")]
    Expired,
    #[error("Invalid password token")]
    InvalidPwdToken,
}

impl ErrToResponse for TokenError {
    fn error(&self) -> &'static str {
        "TokenError"
    }

    fn err_type(&self) -> &'static str {
        match self {
            Self::InternalError(_) => stringify!(InternalError),
            Self::NotFound => stringify!(NotFound),
            Self::Expired => stringify!(Expired),
            Self::InvalidPwdToken => stringify!(InvalidPwdToken),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> HttpResponseBuilder {
        match self {
            Self::InternalError(_) => HttpResponse::InternalServerError(),
            Self::NotFound => HttpResponse::NotFound(),
            Self::Expired => HttpResponse::Gone(),
            Self::InvalidPwdToken => HttpResponse::Forbidden(),
        }
    }

    fn handle(&self) {
        if let Self::InternalError(err) = self {
            log::error!("An internal server error occurred while handling token: {err}");
        }
    }
}
