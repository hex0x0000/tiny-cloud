// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::validate_user;
use crate::config;
use crate::database;
use crate::token::{self, error::TokenError};
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, get, post, web};
use async_sqlite::Pool;
use common_library::error::ErrToResponse;
use common_library::serde_json::{Value, json};
use serde::Deserialize;

/// New token information
#[derive(Deserialize)]
#[non_exhaustive]
pub struct NewToken {
    pub duration: Option<u64>,
    pub for_user: Option<String>,
}

#[derive(Deserialize)]
struct TokenInfo {
    id: Option<i64>,
    token: Option<String>,
}

async fn is_admin(pool: &Pool, user: Identity) -> Result<(), HttpResponse> {
    match validate_user(pool, user).await {
        Ok((_, is_admin)) if is_admin => Ok(()),
        Ok(_) => Err(HttpResponse::Forbidden().body("")),
        Err(e) => Err(e.to_response()),
    }
}

/// Creates a new token
#[post("/new")]
pub async fn new(user: Identity, pool: web::Data<Pool>, info: web::Json<NewToken>) -> impl Responder {
    if let Some(registration) = config!(registration) {
        let pool = pool.into_inner();
        if let Err(e) = is_admin(&pool, user).await {
            return e;
        }
        match database::token::create_token(&pool, registration, info.into_inner()).await {
            Ok((token, duration)) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json!({"token": token, "duration": duration}).to_string()),
            Err(e) => Into::<TokenError>::into(e).to_response(),
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}

/// Deletes a token
#[post("/delete")]
pub async fn delete(user: Identity, pool: web::Data<Pool>, token: web::Json<TokenInfo>) -> impl Responder {
    if config!(registration).is_some() {
        let pool = pool.into_inner();
        let token = token.into_inner();
        if let Err(e) = is_admin(&pool, user).await {
            return e;
        }
        if let Err(e) = token::remove_token(&pool, token.id, token.token).await {
            return e.to_response();
        }
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::NotFound().body("")
    }
}

/// Returns a list of every token with their expire dates
#[get("/list")]
pub async fn list(user: Identity, pool: web::Data<Pool>) -> impl Responder {
    if config!(registration).is_some() {
        let pool = pool.into_inner();
        if let Err(e) = is_admin(&pool, user).await {
            return e;
        }
        match token::get_all_tokens(&pool).await {
            Ok(tokens) => HttpResponse::Ok()
                .content_type("application/json")
                .body(Value::Array(tokens).to_string()),
            Err(e) => e.to_response(),
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}
