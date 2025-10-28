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

pub mod error;
use crate::database;
use async_sqlite::Pool;
use common_library::serde_json::{Value, json};
use database::{token, utils};
use error::TokenError;

/// Checks token to create a new account.
pub async fn check_token(pool: &Pool, token: String) -> Result<(), TokenError> {
    let db_token = token::get_token(pool, token.clone())
        .await
        .map_err(|e| TokenError::InternalError(e.to_string()))?
        .ok_or(TokenError::NotFound)?;
    // Will panic in 292 billion years, be ready for that year!
    let now = utils::now().map_err(|e| e.into())? as i64;
    if db_token.expire_date < now {
        token::remove_expired_tokens(pool).await.map_err(|e| e.into())?;
        return Err(TokenError::Expired);
    }
    token::delete_token(pool, token).await.map_err(|e| e.into())?;
    Ok(())
}

/// Checks token to change password.
pub async fn check_pwd_token(pool: &Pool, token: String, user: String) -> Result<(), TokenError> {
    let db_token = token::get_token(pool, token.clone())
        .await
        .map_err(|e| TokenError::InternalError(e.to_string()))?
        .ok_or(TokenError::NotFound)?;
    let now = utils::now().map_err(|e| e.into())? as i64;
    if db_token.expire_date < now {
        token::remove_expired_tokens(pool).await.map_err(|e| e.into())?;
        return Err(TokenError::Expired);
    }
    if db_token.for_user.map(|u| u != user).unwrap_or(true) {
        return Err(TokenError::InvalidPwdToken);
    }
    token::delete_token(pool, token).await.map_err(|e| e.into())?;
    Ok(())
}

pub async fn remove_token(pool: &Pool, id: Option<i64>, token: Option<String>) -> Result<(), TokenError> {
    if let Some(id) = id {
        token::delete_token_by_id(pool, id).await.map_err(|e| e.into())?;
    } else if let Some(token) = token {
        token::delete_token(pool, token).await.map_err(|e| e.into())?;
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
