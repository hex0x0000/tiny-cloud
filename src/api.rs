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

#[macro_use]
mod macros;
pub mod auth;
pub mod plugins;
pub mod token;
use crate::config;
use actix_web::{get, HttpResponse, Responder};
use std::sync::OnceLock;
use tcloud_library::serde_json::json;

static INFO: OnceLock<String> = OnceLock::new();

/// Returns server info
#[get("/info")]
pub async fn info() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(
        INFO.get_or_init(|| {
            json!({
                "name": config!(server_name),
                "version": env!("CARGO_PKG_VERSION"),
                "description": config!(description),
                "source": env!("CARGO_PKG_REPOSITORY")
            })
            .to_string()
        })
        .to_owned(),
    )
}
