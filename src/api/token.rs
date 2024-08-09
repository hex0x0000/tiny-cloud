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

use crate::config;
use crate::database;
use crate::token::{self, error::TokenError};
use actix_identity::error::GetIdentityError;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use async_sqlite::Pool;
use serde::Deserialize;
use tcloud_library::error::ErrToResponse;
use tcloud_library::serde_json::{json, Value};

/// New token duration
#[derive(Deserialize)]
struct NewToken {
    duration: Option<u64>,
}

#[derive(Deserialize)]
struct TokenInfo {
    id: Option<i64>,
    token: Option<String>,
}

async fn check_admin(pool: &Pool, username: String) -> Result<(), HttpResponse> {
    if let Some(is_admin) = database::auth::is_admin(pool, username)
        .await
        .map_err(|e| Into::<TokenError>::into(e).to_response())?
    {
        if is_admin {
            Ok(())
        } else {
            Err(HttpResponse::Forbidden().body(""))
        }
    } else {
        Err(HttpResponse::Forbidden().body(""))
    }
}

/// Creates a new token
#[post("/new")]
pub async fn new(user: Identity, pool: web::Data<Pool>, info: web::Json<NewToken>) -> impl Responder {
    if let Some(registration) = config!(registration) {
        let pool = pool.into_inner();
        let username = get_user!(user.id());
        if let Err(e) = check_admin(&pool, username).await {
            return e;
        }
        match database::token::create_token(&pool, registration, info.into_inner().duration).await {
            Ok((token, duration)) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json!({"token": token, "duration": duration}).to_string()),
            Err(e) => Into::<TokenError>::into(e).to_response(),
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}

#[post("/delete")]
pub async fn delete(user: Identity, pool: web::Data<Pool>, token: web::Json<TokenInfo>) -> impl Responder {
    if config!(registration).is_some() {
        let username = get_user!(user.id());
        let pool = pool.into_inner();
        let token = token.into_inner();
        if let Err(e) = check_admin(&pool, username).await {
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
        let username = get_user!(user.id());
        let pool = pool.into_inner();
        if let Err(e) = check_admin(&pool, username).await {
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
