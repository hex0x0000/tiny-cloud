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

use crate::{
    auth::{self, error::AuthError},
    config,
    utils::{get_ip, sanitize_user},
};
use actix_identity::error::GetIdentityError;
use actix_identity::Identity;
use actix_web::{dev::ConnectionInfo, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use async_sqlite::Pool;
use serde::Deserialize;
use tcloud_library::error::ErrToResponse;
use zeroize::Zeroizing;

/// Username and password sent by the client to login.
#[non_exhaustive]
#[derive(Deserialize)]
pub struct Login {
    pub user: String,
    pub password: String,
    #[cfg(feature = "totp-auth")]
    pub totp: String,
}

/// Username, password and token sent by the client to register
#[derive(Deserialize)]
pub struct Register {
    user: String,
    password: String,
    token: String,
    #[cfg(feature = "totp-auth")]
    totp_as_qr: bool,
}

/// Registers new user and starts a new session
#[cfg(not(feature = "totp-auth"))]
#[post("/register")]
pub async fn register(
    req: HttpRequest,
    conn: ConnectionInfo,
    credentials: web::Json<Register>,
    pool: web::Data<Pool>,
) -> impl Responder {
    if config!(registration).is_some() {
        let credentials = credentials.into_inner();
        let password = Zeroizing::new(credentials.password.into_bytes());
        let pool = pool.into_inner();
        match auth::register_user(&pool, credentials.user.clone(), password, credentials.token).await {
            Ok(_) => {
                if let Err(err) = Identity::login(&req.extensions(), credentials.user.clone()) {
                    return AuthError::InternalError(format!("Failed to build identity during registration: {err}")).to_response();
                }
                log::warn!("client [{}] registered as `{}`", get_ip(&conn), sanitize_user(&credentials.user));
                HttpResponse::Ok().body("")
            }
            Err(err) => {
                log::warn!(
                    "client [{}] tried to register as `{}`",
                    get_ip(&conn),
                    sanitize_user(&credentials.user)
                );
                err.to_response()
            }
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}

/// Registers new user and starts a new session.
/// Returns the TOTP as a url or qr code depending on the request
#[cfg(feature = "totp-auth")]
#[post("/register")]
pub async fn register(
    req: HttpRequest,
    conn: ConnectionInfo,
    credentials: web::Json<Register>,
    pool: web::Data<Pool>,
) -> impl Responder {
    use tcloud_library::serde_json::json;

    if config!(registration).is_some() {
        let credentials = credentials.into_inner();
        let password = Zeroizing::new(credentials.password.into_bytes());
        let pool = pool.into_inner();
        match auth::register_user(&pool, credentials.user.clone(), password, credentials.token).await {
            Ok(totp) => {
                if let Err(err) = Identity::login(&req.extensions(), credentials.user.clone()) {
                    return AuthError::InternalError(format!("Failed to build identity during registration: {err}")).to_response();
                }
                log::warn!("client [{}] registered as `{}`", get_ip(&conn), sanitize_user(&credentials.user));
                let mut resp = HttpResponse::Ok();
                resp.content_type("application/json");
                if credentials.totp_as_qr {
                    match totp.get_qr_base64() {
                        Ok(qr) => resp.body(json!({ "totp_qr": qr }).to_string()),
                        Err(e) => AuthError::InternalError(format!("Failed to get TOTP QR code image as base64: {e}")).to_response(),
                    }
                } else {
                    resp.body(json!({ "totp_url": totp.get_url() }).to_string())
                }
            }
            Err(err) => {
                log::warn!(
                    "client [{}] tried to register as `{}`",
                    get_ip(&conn),
                    sanitize_user(&credentials.user)
                );
                err.to_response()
            }
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}

/// Logins and starts a new session
#[post("/login")]
pub async fn login(req: HttpRequest, conn: ConnectionInfo, login: web::Json<Login>, pool: web::Data<Pool>) -> impl Responder {
    let login = login.into_inner();
    let pool = pool.into_inner();
    match auth::check(&pool, login).await {
        Ok(user) => {
            log::warn!("client [{}] logged in as `{}`", get_ip(&conn), sanitize_user(&user));
            if let Err(err) = Identity::login(&req.extensions(), user) {
                return AuthError::InternalError(format!("Failed to build identity during registration: {err}")).to_response();
            }
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            log::warn!("client [{}] failed to login", get_ip(&conn));
            err.to_response()
        }
    }
}

/// Logs out and ends current session
#[get("/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

// Deletes an user's own account
#[get("/delete")]
pub async fn delete(user: Identity, pool: web::Data<Pool>) -> impl Responder {
    let username = get_user!(user.id());
    let pool = pool.into_inner();
    user.logout();
    if let Err(err) = auth::delete_user(&pool, username).await {
        err.to_response()
    } else {
        HttpResponse::Ok().body("")
    }
}
