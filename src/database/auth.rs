use super::error::DBError;
use async_sqlite::{
    rusqlite::{self, named_params, ErrorCode, OptionalExtension},
    Error, Pool,
};

#[non_exhaustive]
pub struct User {
    pub id: i64,
    pub username: String,
    pub pass_hash: String,
    #[cfg(feature = "totp-auth")]
    pub totp: String,
    pub is_admin: bool,
}

#[cfg(not(feature = "totp-auth"))]
pub const USERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY,
    username    TEXT    NOT NULL,
    pass_hash   TEXT    NOT NULL,
    is_admin    INTEGER DEFAULT 0,
    UNIQUE(username)
);";

#[cfg(feature = "totp-auth")]
pub const USERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY,
    username    TEXT    NOT NULL,
    pass_hash   TEXT    NOT NULL,
    totp        TEXT    NOT NULL,
    is_admin    INTEGER DEFAULT 0,
    UNIQUE(username)
);";

#[cfg(not(feature = "totp-auth"))]
const INSERT_USER: &str =
    "INSERT INTO users (username, pass_hash, is_admin) VALUES (:username, :pass_hash, :is_admin)";

#[cfg(feature = "totp-auth")]
const INSERT_USER: &str = "INSERT INTO users (username, pass_hash, totp, is_admin) VALUES (:username, :pass_hash, :totp, :is_admin)";

/// Adds a new user to the database, fails if it already exists.
/// If TOTP feature is enabled, it requires the totp-secret to be inserted
pub async fn add_user(
    pool: &Pool,
    username: String,
    pass_hash: String,
    #[cfg(feature = "totp-auth")] totp: String,
    is_admin: bool,
) -> Result<(), DBError> {
    pool.conn(move |conn| {
        conn.execute(
            INSERT_USER,
            #[cfg(not(feature = "totp-auth"))]
            named_params! {
                ":username": username,
                ":pass_hash": pass_hash,
                ":is_admin": is_admin,
            },
            #[cfg(feature = "totp-auth")]
            named_params! {
                ":username": username,
                ":pass_hash": pass_hash,
                ":totp": totp,
                ":is_admin": is_admin,
            },
        )
    })
    .await
    .map_err(|e| {
        if let Error::Rusqlite(ref err) = e {
            if let rusqlite::Error::SqliteFailure(err, _) = err {
                if err.code == ErrorCode::ConstraintViolation {
                    return DBError::UserExists;
                }
            }
        }
        DBError::ExecError(format!("Failed to insert user: {e}"))
    })?;
    Ok(())
}

#[cfg(not(feature = "totp-auth"))]
fn get_user_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<User> {
    Ok(User {
        id: row.get(0)?,
        username: row.get(1)?,
        pass_hash: row.get(2)?,
        is_admin: row.get(3)?,
    })
}

#[cfg(feature = "totp-auth")]
fn get_user_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<User> {
    Ok(User {
        id: row.get(0)?,
        username: row.get(1)?,
        pass_hash: row.get(2)?,
        totp: row.get(3)?,
        is_admin: row.get(4)?,
    })
}

/// Returns a user. User contains the TOTP secret depending on wether or not
/// the "totp-auth" feature is enabled.
pub async fn get_user(pool: &Pool, username: String) -> Result<Option<User>, DBError> {
    pool.conn(|conn| {
        conn.query_row(
            "SELECT * FROM users WHERE username=?1",
            [username],
            get_user_row,
        )
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Deletes a user from database
pub async fn delete_user(pool: &Pool, username: String) -> Result<(), DBError> {
    pool.conn(move |conn| conn.execute("DELETE FROM users WHERE username=?1", [username]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to delete user: {e}")))?;
    Ok(())
}
