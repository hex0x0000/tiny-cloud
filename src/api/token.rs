use crate::config;
use crate::database::{self, auth};
use crate::error::ErrToResponse;
use crate::token::{self, error::TokenError};
use actix_identity::error::GetIdentityError;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use async_sqlite::Pool;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

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
    if let Some(user) = auth::get_user(pool, username)
        .await
        .map_err(|e| Into::<TokenError>::into(e).to_response())?
    {
        if user.is_admin {
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
pub async fn new(
    user: Identity,
    pool: web::Data<Pool>,
    info: web::Json<NewToken>,
) -> impl Responder {
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
pub async fn delete(
    user: Identity,
    pool: web::Data<Pool>,
    token: web::Json<TokenInfo>,
) -> impl Responder {
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
