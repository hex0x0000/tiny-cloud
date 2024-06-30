pub mod error;
use crate::database;
use async_sqlite::Pool;
use database::{token, utils};
use error::TokenError;
use serde_json::{json, Value};

pub async fn check_token(pool: &Pool, token: String) -> Result<(), TokenError> {
    let db_token = token::get_token(pool, token.clone())
        .await
        .map_err(|e| TokenError::InternalError(e.to_string()))?
        .ok_or(TokenError::NotFound)?;
    if db_token.token == token {
        // Will panic in 292 billion years, be ready for that year!
        let now = utils::now().map_err(|e| e.into())? as i64;
        if db_token.expire_date < now {
            token::remove_expired_tokens(pool)
                .await
                .map_err(|e| e.into())?;
            return Err(TokenError::Expired);
        }
    } else {
        return Err(TokenError::NotFound);
    }
    token::delete_token(pool, token)
        .await
        .map_err(|e| e.into())?;
    Ok(())
}

pub async fn remove_token(
    pool: &Pool,
    id: Option<i64>,
    token: Option<String>,
) -> Result<(), TokenError> {
    if let Some(id) = id {
        token::delete_token_by_id(pool, id)
            .await
            .map_err(|e| e.into())?;
    } else if let Some(token) = token {
        token::delete_token(pool, token)
            .await
            .map_err(|e| e.into())?;
    }
    Ok(())
}

pub async fn get_all_tokens(pool: &Pool) -> Result<Vec<Value>, TokenError> {
    token::get_all_tokens(pool)
        .await
        .map(|v| {
            v.iter()
                .map(|t| json! ({"id": t.id, "token": t.token.clone(), "expire": t.expire_date}))
                .collect::<Vec<Value>>()
        })
        .map_err(|e| e.into())
}
