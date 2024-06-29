use super::{
    error::DBError,
    utils::{calc_expire, now},
};
use crate::config::Registration;
use async_sqlite::{
    rusqlite::{named_params, OptionalExtension},
    Pool,
};
use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;

#[non_exhaustive]
pub struct Token {
    pub id: i64,
    pub token: String,
    pub expire_date: i64,
}

pub const TOKEN_TABLE: &str = "
CREATE TABLE IF NOT EXISTS tokens (
    id          INTEGER PRIMARY KEY,
    token       TEXT    NOT NULL,
    expire_date INTEGER NOT NULL,
    UNIQUE(token)
);";

const INSERT_TOKEN: &str = "INSERT INTO tokens (token, expire_date) VALUES (:token, :expire_date)";

/// Creates a token and adds it to the database
/// Optionally takes an `duration_secs` param which specifies the duration, if none
/// is given then the config's token_duration_seconds is used
pub async fn create_token(
    pool: &Pool,
    registration: &Registration,
    duration_secs: Option<u64>,
) -> Result<(String, u64), DBError> {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(registration.token_size.into())
        .map(char::from)
        .collect();
    let _token = token.clone();
    let duration = if let Some(duration) = duration_secs {
        duration
    } else {
        registration.token_duration_seconds
    };
    let expire_date: u64 = calc_expire(Duration::new(duration, 0))?;
    pool.conn(move |conn| {
        conn.execute(
            INSERT_TOKEN,
            named_params! {
                ":token": token,
                ":expire_date": expire_date,
            },
        )
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to create token: {e}")))?;
    Ok((_token, duration))
}

/// Gets token's data (id and expire date) if it exists
pub async fn get_token(pool: &Pool, token: String) -> Result<Option<Token>, DBError> {
    Ok(pool
        .conn(|conn| {
            conn.query_row("SELECT * FROM tokens WHERE token=?1", [token], |row| {
                Ok(Token {
                    id: row.get(0)?,
                    token: row.get(1)?,
                    expire_date: row.get(2)?,
                })
            })
            .optional()
        })
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to get token: {e}")))?)
}

/// Gets all saved tokens
pub async fn get_all_tokens(pool: &Pool) -> Result<Vec<Token>, DBError> {
    Ok(pool
        .conn(|conn| {
            let mut stmt = conn.prepare("SELECT * FROM tokens")?;
            let rows = stmt.query_map([], |row| {
                Ok(Token {
                    id: row.get(0)?,
                    token: row.get(1)?,
                    expire_date: row.get(2)?,
                })
            })?;
            rows.collect()
        })
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to get tokens: {e}")))?)
}

/// Removes a token
pub async fn delete_token(pool: &Pool, token: String) -> Result<(), DBError> {
    pool.conn(move |conn| conn.execute("DELETE FROM tokens WHERE token = ?1", [token]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to remove token: {e}")))?;
    Ok(())
}

/// Removes a token by its ID
pub async fn delete_token_by_id(pool: &Pool, id: i64) -> Result<(), DBError> {
    pool.conn(move |conn| conn.execute("DELETE FROM tokens WHERE id = ?1", [id]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to remove token by id: {e}")))?;
    Ok(())
}

/// Removes all expired tokens
pub async fn remove_expired_tokens(pool: &Pool) -> Result<(), DBError> {
    let now = now()?;
    pool.conn(move |conn| conn.execute("DELETE FROM tokens WHERE expire_date < ?1", [now]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to remove expired tokens: {e}")))?;
    Ok(())
}
