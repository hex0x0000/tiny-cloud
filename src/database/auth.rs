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
    rusqlite::{self, named_params, params, ErrorCode, OptionalExtension},
    Error, Pool,
};

/// Authentication data of a user.
#[non_exhaustive]
pub struct UserAuth {
    /// Username and id used as session identity. Formatted as "USERNAME:ID"
    pub userid: String,
    /// Password's hash of the user.
    pub pass_hash: String,
    /// TOTP secret of the user. Enabled only with feature "totp-auth".
    pub totp: String,
}

pub const USERS_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
    id          BIGINT  UNIQUE PRIMARY KEY,
    username    TEXT    UNIQUE NOT NULL,
    pass_hash   TEXT    NOT NULL,
    totp        TEXT    NOT NULL,
    is_admin    INTEGER DEFAULT 0
)";

const INSERT_USER: &str =
    "INSERT INTO users (id, username, pass_hash, totp, is_admin) VALUES (:id, :username, :pass_hash, :totp, :is_admin)";

const GET_USER_AUTH: &str = "SELECT id, username, pass_hash, totp FROM users WHERE username=?1";

/// Adds a new user to the database, fails if it already exists.
/// If TOTP feature is enabled, it requires the totp-secret to be inserted.
/// Returns a string containing the username and the id of the user. It is used as the identifier
/// during a session. Formatted as "USERNAME:ID".
pub async fn add_user(pool: &Pool, username: String, pass_hash: String, totp: String, is_admin: bool) -> Result<String, DBError> {
    let username_clone = username.clone();
    let id: i64 = i64::abs(rand::random());
    pool.conn(move |conn| {
        conn.execute(
            INSERT_USER,
            named_params! {
                ":id": id,
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
    Ok(format!("{username}:{id}"))
}

/// Returns a user's authentication data as a [`UserAuth`].
pub async fn get_auth(pool: &Pool, username: String) -> Result<Option<UserAuth>, DBError> {
    pool.conn(|conn| {
        conn.query_row(GET_USER_AUTH, [username], |row| {
            Ok(UserAuth {
                userid: format!("{}:{}", row.get::<usize, String>(1)?, row.get::<usize, i64>(0)?),
                pass_hash: row.get(2)?,
                totp: row.get(3)?,
            })
        })
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Unpacks the userid into the username and the id.
fn unpack(userid: String) -> Result<(i64, String), ()> {
    let split: Vec<&str> = userid.split(':').collect();
    Ok((split[1].parse().map_err(|_| ())?, split[0].into()))
}

/// Returns username and user admin status from userid.
/// If the userid is not valid returns [`None`].
pub async fn userinfo(pool: &Pool, userid: String) -> Result<Option<(String, bool)>, DBError> {
    let (id, username) = unpack(userid).map_err(|_| DBError::InvalidUserID)?;
    pool.conn(move |conn| {
        conn.query_row(
            "SELECT username, is_admin FROM users WHERE id=?1 AND username=?2",
            params![id, username],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
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

/// Deletes user from database and all of its
pub async fn delete_user(pool: &Pool, userid: String) -> Result<(), DBError> {
    let (id, username) = unpack(userid).map_err(|_| DBError::InvalidUserID)?;
    let user = username.clone();
    pool.conn(move |conn| conn.execute("DELETE FROM users WHERE id=?1 AND username=?2", params![id, username]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to delete user '{user}': {e}")))?;
    log::info!("Deleted user '{user}'");
    super::delete_user_dir(&user).await?;
    log::info!("Deleted user directory of '{user}'");
    Ok(())
}
