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

use super::error::DBError;
use async_sqlite::{
    rusqlite::{self, named_params, ErrorCode, OptionalExtension},
    Error, Pool,
};

/// Authentication data of a user.
#[non_exhaustive]
pub struct UserAuth {
    /// Password's hash of the user.
    pub pass_hash: String,
    /// TOTP secret of the user. Enabled only with feature "totp-auth".
    #[cfg(feature = "totp-auth")]
    pub totp: String,
}

#[cfg(not(feature = "totp-auth"))]
pub const USERS_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY,
    username    TEXT    NOT NULL,
    pass_hash   TEXT    NOT NULL,
    is_admin    INT     DEFAULT 0,
    UNIQUE(username)
)";

#[cfg(feature = "totp-auth")]
pub const USERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY,
    username    TEXT    NOT NULL,
    pass_hash   TEXT    NOT NULL,
    totp        TEXT    NOT NULL,
    is_admin    INTEGER DEFAULT 0,
    UNIQUE(username)
)";

#[cfg(not(feature = "totp-auth"))]
const INSERT_USER: &str = "INSERT INTO users (username, pass_hash, is_admin) VALUES (:username, :pass_hash, :is_admin)";

#[cfg(feature = "totp-auth")]
const INSERT_USER: &str = "INSERT INTO users (username, pass_hash, totp, is_admin) VALUES (:username, :pass_hash, :totp, :is_admin)";

#[cfg(not(feature = "totp-auth"))]
const GET_USER_AUTH: &str = "SELECT pass_hash FROM users WHERE username=?1";

#[cfg(feature = "totp-auth")]
const GET_USER_AUTH: &str = "SELECT pass_hash, totp FROM users WHERE username=?1";

/// Adds a new user to the database, fails if it already exists.
/// If TOTP feature is enabled, it requires the totp-secret to be inserted
pub async fn add_user(
    pool: &Pool,
    username: String,
    pass_hash: String,
    #[cfg(feature = "totp-auth")] totp: String,
    is_admin: bool,
) -> Result<(), DBError> {
    let username_clone = username.clone();
    pool.conn(move |conn| {
        conn.execute(
            INSERT_USER,
            #[cfg(not(feature = "totp-auth"))]
            named_params! {
                ":username": username_clone,
                ":pass_hash": pass_hash,
                ":is_admin": is_admin,
            },
            #[cfg(feature = "totp-auth")]
            named_params! {
                ":username": username_clone,
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
    log::info!("Added a new user: '{username}'");
    super::create_user_dir(&username).await?;
    log::info!("Created new user dir for '{username}'");
    Ok(())
}

/// Returns a user's authentication data as a [`UserAuth`].
pub async fn get_auth(pool: &Pool, username: String) -> Result<Option<UserAuth>, DBError> {
    pool.conn(|conn| {
        conn.query_row(GET_USER_AUTH, [username], |row| {
            Ok(UserAuth {
                pass_hash: row.get(0)?,
                #[cfg(feature = "totp-auth")]
                totp: row.get(1)?,
            })
        })
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Returns whether or not a user is an admin. If the user does not exist returns [`None`].
pub async fn is_admin(pool: &Pool, username: String) -> Result<Option<bool>, DBError> {
    pool.conn(|conn| {
        conn.query_row("SELECT is_admin FROM users WHERE username=?1", [username], |row| row.get(0))
            .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Gets a list of all the usernames in the database
pub async fn get_all_usernames(pool: &Pool) -> Result<Vec<String>, DBError> {
    pool.conn(|conn| {
        let mut stmt = conn.prepare("SELECT username FROM users")?;
        let mut rows = stmt.query([])?;
        let mut users: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            users.push(row.get(0)?);
        }
        Ok(users)
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Deletes a user from database
pub async fn delete_user(pool: &Pool, username: String) -> Result<(), DBError> {
    let username_clone = username.clone();
    pool.conn(move |conn| conn.execute("DELETE FROM users WHERE username=?1", [username_clone]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to delete user: {e}")))?;
    log::info!("Deleted user '{username}'");
    super::delete_user_dir(&username).await?;
    log::info!("Deleted user directory of '{username}'");
    Ok(())
}
