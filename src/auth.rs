pub mod cli;
mod database;
pub mod error;
mod hash;
mod utils;

use crate::config;
use async_sqlite::Pool;
use error::{AuthError, DBError};

use self::utils::now;

fn check_validity(username: &String, password: &Vec<u8>) -> Result<(), AuthError> {
    let user_len = username.len();
    let passwd_len = password.len();
    let max_username_size = *config!(max_username_size) as usize;
    let min_username_size = *config!(min_username_size) as usize;
    let max_passwd_size = *config!(max_passwd_size) as usize;
    let min_passwd_size = *config!(min_passwd_size) as usize;
    if user_len > max_username_size || user_len < min_username_size {
        return Err(AuthError::BadCredentials(format!(
            "Accepted username size is between {min_username_size} and {max_username_size} characters",
        )));
    }
    if passwd_len > max_passwd_size || passwd_len < min_passwd_size {
        return Err(AuthError::BadCredentials(format!(
            "Accepted password length is between {min_passwd_size} and {max_passwd_size} bytes",
        )));
    }
    for c in username.chars() {
        if !c.is_alphanumeric() {
            return Err(AuthError::BadCredentials(
                "Username must be alphanumerical".into(),
            ));
        }
    }
    Ok(())
}

async fn check_token(pool: &Pool, token: String) -> Result<(), AuthError> {
    let db_token = database::get_token(pool, token.clone())
        .await
        .map_err(|e| AuthError::InternalError(e.to_string()))?
        .ok_or(AuthError::BadToken)?;
    if db_token.token == token {
        // Will panic in 292 billion years, be ready for that year!
        let now = now().map_err(|e| e.into())? as i64;
        if db_token.expire_date < now {
            database::remove_expired_tokens(pool)
                .await
                .map_err(|e| e.into())?;
            return Err(AuthError::BadToken);
        }
    } else {
        return Err(AuthError::BadToken);
    }
    database::delete_token(pool, token)
        .await
        .map_err(|e| e.into())?;
    Ok(())
}

pub async fn init_db() -> Option<Pool> {
    match database::init_db().await {
        Ok(pool) => Some(pool),
        Err(e) => {
            log::error!("Failed to init auth database: {e}");
            None
        }
    }
}

/// Checks a user's password
pub async fn check(pool: &Pool, username: &String, password: &Vec<u8>) -> Result<(), AuthError> {
    check_validity(username, password)?;
    let hash = database::get_user(pool, username.clone())
        .await
        .map_err(|e| e.into())?
        .ok_or(AuthError::InvalidCredentials)?
        .pass_hash;
    hash::verify(password, &hash)
}

/// Adds a new user. Fails if username already exists
pub async fn add_user(
    pool: &Pool,
    username: String,
    password: &Vec<u8>,
    is_admin: bool,
) -> Result<(), AuthError> {
    check_validity(&username, password)?;
    let passwd_hash = hash::create(password)?;
    database::add_user(pool, username, passwd_hash, is_admin)
        .await
        .map_err(|e| e.into())?;
    Ok(())
}

/// Registers a new user with a token.
/// Fails if username already exists or if token is not valid
pub async fn register_user(
    pool: &Pool,
    username: String,
    password: &Vec<u8>,
    token: String,
) -> Result<(), AuthError> {
    check_validity(&username, password)?;
    check_token(pool, token).await?;
    let passwd_hash = hash::create(password)?;
    database::add_user(pool, username, passwd_hash, false)
        .await
        .map_err(|e| e.into())?;
    Ok(())
}

pub async fn delete_user(pool: &Pool, username: String) -> Result<(), AuthError> {
    database::delete_user(&pool, username)
        .await
        .map_err(|e| e.into())?;
    Ok(())
}
