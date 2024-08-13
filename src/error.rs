// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use actix_web::{HttpResponse, HttpResponseBuilder};
use tcloud_library::error::ErrToResponse;
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
