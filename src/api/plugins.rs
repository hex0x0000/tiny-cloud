// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use actix_web::{Responder, post, web};
use async_sqlite::Pool;
use common_library::{Json, error::ErrToResponse, plugin::User};

use crate::{auth::validate_user, plugins::Plugins};

#[derive(MultipartForm)]
pub struct FileForm {
    pub file: TempFile,
    pub info: MpJson<Json>,
}

/// Handles plugins
#[post("/p/{plugin}")]
pub async fn handler(
    pool: web::Data<Pool>,
    plugin: web::Path<String>,
    body: web::Json<Json>,
    plugins: web::Data<Plugins>,
    user: Option<Identity>,
) -> impl Responder {
    let pool = pool.into_inner();
    let plugin = plugin.into_inner();
    let plugins = plugins.into_inner();
    let body = body.into_inner();
    let user: Option<User> = match user {
        Some(user) => match validate_user(&pool, user).await {
            Ok((username, is_admin)) => Some(User { name: username, is_admin }),
            Err(e) => return e.to_response(),
        },
        None => None,
    };
    plugins.request(plugin, user, body).await
}

/// Handles file uploading for plugins
#[post("/up/{plugin}")]
pub async fn file(
    pool: web::Data<Pool>,
    plugins: web::Data<Plugins>,
    plugin: web::Path<String>,
    user: Option<Identity>,
    MultipartForm(form): MultipartForm<FileForm>,
) -> impl Responder {
    let pool = pool.into_inner();
    let plugin = plugin.into_inner();
    let plugins = plugins.into_inner();
    let user: Option<User> = match user {
        Some(user) => match validate_user(&pool, user).await {
            Ok((username, is_admin)) => Some(User { name: username, is_admin }),
            Err(e) => return e.to_response(),
        },
        None => None,
    };
    plugins.file(plugin, user, form).await
}
