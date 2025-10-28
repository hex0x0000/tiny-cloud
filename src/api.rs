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


pub mod auth;
pub mod plugins;
pub mod token;
use crate::config;
use actix_web::{HttpResponse, Responder, get};
use common_library::serde_json::json;
use std::sync::LazyLock;

static INFO: LazyLock<String> = LazyLock::new(|| {
    json!({
        "name": config!(server_name),
        "version": env!("CARGO_PKG_VERSION"),
        "description": config!(description),
        "source": env!("CARGO_PKG_REPOSITORY"),
        "plugins": crate::plugins::list()
    })
    .to_string()
});

/// Returns server info
#[get("/info")]
pub async fn info() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(INFO.to_owned())
}
