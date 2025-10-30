// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    auth::{self, error::AuthError},
    config,
    utils::{get_ip, sanitize_user},
};
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, dev::ConnectionInfo, get, post, web};
use async_sqlite::Pool;
use common_library::error::ErrToResponse;
use serde::Deserialize;
use zeroize::{Zeroize, ZeroizeOnDrop};
use totp_rs::TOTP;

/// Username and password sent by the client to login.
#[non_exhaustive]
#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Login {
    pub user: String,
    pub password: String,
    pub totp: String,
}

/// Username, password and token sent by the client to register
#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Register {
    user: String,
    password: String,
    token: String,
    totp_as_qr: bool,
}

/// Method to change user's password
#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
#[serde(rename_all = "lowercase")]
pub enum ChangeMethod {
    /// A token must be given from the admin to user in order to change the user's password,
    /// if the user doesn't remember it
    Token(String),
    /// The user can use the old password to change the new password
    OldPassword(String),
}

/// Payload to change the user's password
#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct ChangePwd {
    new_password: String,
    #[serde(flatten)]
    change_method: ChangeMethod,
}

/// Payload to change user's TOTP secret
#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct ChangeTotp {
    password: String,
    totp_as_qr: bool
}

fn return_totp_response(totp: TOTP, as_qr: bool) -> HttpResponse {
    use common_library::serde_json::json;
    let mut resp = HttpResponse::Ok();
    resp.content_type("application/json");
    if as_qr {
        match totp.get_qr_base64() {
            Ok(qr) => resp.body(json!({ "totp_qr": qr }).to_string()),
            Err(e) => AuthError::InternalError(format!("Failed to get TOTP QR code image as base64: {e}")).to_response(),
        }
    } else {
        resp.body(json!({ "totp_url": totp.get_url() }).to_string())
    }
}

/// Registers new user and starts a new session.
/// Returns the TOTP as a url or qr code depending on the request
#[post("/register")]
pub async fn register(
    req: HttpRequest,
    conn: ConnectionInfo,
    credentials: web::Json<Register>,
    pool: web::Data<Pool>,
) -> impl Responder {
    if config!(registration).is_some() {
        let credentials = credentials.into_inner();
        let pool = pool.into_inner();
        match auth::register_user(
            &pool,
            credentials.user.clone(),
            credentials.password.as_bytes(),
            credentials.token.clone(),
        )
        .await
        {
            Ok((totp, userid)) => {
                log::warn!("client [{}] registered as `{}`", get_ip(&conn), sanitize_user(&credentials.user));
                if let Err(err) = Identity::login(&req.extensions(), userid) {
                    return AuthError::InternalError(format!("Failed to build identity during registration: {err}")).to_response();
                }
                return_totp_response(totp, credentials.totp_as_qr)
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

/// Changes session id and causes all active sessions to logout.
#[get("/logoutall")]
pub async fn logoutall(user: Identity, pool: web::Data<Pool>) -> impl Responder {
    let pool = pool.into_inner();
    if let Err(err) = auth::change_sessionid(&pool, user).await {
        err.to_response()
    } else {
        HttpResponse::Ok().body("")
    }
}

// Deletes an user's own account
#[get("/delete")]
pub async fn delete(user: Identity, pool: web::Data<Pool>) -> impl Responder {
    let pool = pool.into_inner();
    if let Err(err) = auth::delete_user(&pool, user).await {
        err.to_response()
    } else {
        HttpResponse::Ok().body("")
    }
}

/// Changes user passwords and invalidates old sessions
#[post("/changepwd")]
pub async fn changepwd(user: Identity, pool: web::Data<Pool>, payload: web::Json<ChangePwd>) -> impl Responder {
    let pool = pool.into_inner();
    let payload = payload.into_inner();
    match &payload.change_method {
        ChangeMethod::OldPassword(old_password) => {
            if let Err(err) = auth::change_pwd(&pool, user, payload.new_password.as_bytes(), old_password.as_bytes()).await {
                return err.to_response();
            }
        }
        ChangeMethod::Token(token) => {
            if let Err(err) = auth::change_pwd_token(&pool, user, payload.new_password.as_bytes(), token.to_owned()).await {
                return err.to_response();
            }
        }
    }
    HttpResponse::Ok().body("")
}

#[post("/changetotp")]
pub async fn changetotp(user: Identity, pool: web::Data<Pool>, payload: web::Json<ChangeTotp>) -> impl Responder {
    let pool = pool.into_inner();
    let payload = payload.into_inner();
    let totp = match auth::change_totp(&pool, user, payload.password.as_bytes()).await {
        Ok(totp) => totp,
        Err(err) => return err.to_response()
    };
    return_totp_response(totp, payload.totp_as_qr)
}
