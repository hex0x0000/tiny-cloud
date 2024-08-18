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

use actix_identity::Identity;
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{post, web, Responder};
use async_sqlite::Pool;
use tcloud_library::{error::ErrToResponse, plugin::User, Json};

use crate::{
    database,
    plugins::{error::PluginError, Plugins},
    utils,
};

#[derive(MultipartForm)]
pub struct FileForm {
    pub file: TempFile,
    pub info: MpJson<Json>,
}

async fn is_admin(pool: &Pool, username: &str) -> Result<bool, PluginError> {
    if let Some(is_admin) = database::auth::is_admin(pool, username.to_string())
        .await
        .map_err(Into::<PluginError>::into)?
    {
        Ok(is_admin)
    } else {
        Err(PluginError::InternalError(format!("User '{username}' not found in DB")))
    }
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
    log::info!("a");
    let pool = pool.into_inner();
    let plugin = plugin.into_inner();
    let plugins = plugins.into_inner();
    let body = body.into_inner();
    let user: Option<User> = match user {
        Some(user) => match user.id() {
            Ok(name) => match is_admin(&pool, &name).await {
                Ok(is_admin) => Some(User { name, is_admin }),
                Err(e) => return e.to_response(),
            },
            Err(err) => return utils::id_err_into(err),
        },
        None => None,
    };
    plugins.request(plugin, user, body).await
}

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
        Some(user) => match user.id() {
            Ok(name) => match is_admin(&pool, &name).await {
                Ok(is_admin) => Some(User { name, is_admin }),
                Err(e) => return e.to_response(),
            },
            Err(err) => return utils::id_err_into(err),
        },
        None => None,
    };
    plugins.file(plugin, user, form).await
}
