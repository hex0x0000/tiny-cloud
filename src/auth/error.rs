// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use actix_web::{HttpResponse, HttpResponseBuilder};
use tcloud_library::error::ErrToResponse;
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
    #[cfg(feature = "totp-auth")]
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
            Self::InternalError(_) => stringify!(InternalError),
            #[cfg(feature = "totp-auth")]
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
            #[cfg(feature = "totp-auth")]
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
