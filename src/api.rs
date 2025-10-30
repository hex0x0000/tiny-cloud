// SPDX-License-Identifier: AGPL-3.0-or-later


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
