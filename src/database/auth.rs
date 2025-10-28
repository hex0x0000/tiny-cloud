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
    Error, Pool,
    rusqlite::{self, ErrorCode, OptionalExtension, named_params, params},
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
    username    TEXT    UNIQUE NOT NULL,
    sessionid   BIGINT  UNIQUE NOT NULL,
    pass_hash   TEXT    NOT NULL,
    totp        TEXT    NOT NULL,
    is_admin    INTEGER DEFAULT 0
)";

/// Adds a new user to the database, fails if it already exists.
/// Returns a string containing the username and the session id of the user (called userid),
/// formatted as "USERNAME:SESSION_ID".
pub async fn add_user(pool: &Pool, username: String, pass_hash: String, totp: String, is_admin: bool) -> Result<String, DBError> {
    let username_clone = username.clone();
    let sessionid: i64 = rand::random();
    pool.conn(move |conn| {
        conn.execute(
            "INSERT INTO users (username, sessionid, pass_hash, totp, is_admin) VALUES (:username, :sessionid, :pass_hash, :totp, :is_admin)",
            named_params! {
                ":username": username_clone,
                ":sessionid": sessionid,
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
    Ok(format!("{username}:{sessionid}"))
}

/// Returns a user's authentication data as a [`UserAuth`].
pub async fn get_auth(pool: &Pool, username: String) -> Result<Option<UserAuth>, DBError> {
    pool.conn(|conn| {
        conn.query_row(
            "SELECT username, sessionid, pass_hash, totp FROM users WHERE username=?1",
            [username],
            |row| {
                Ok(UserAuth {
                    userid: format!("{}:{}", row.get::<usize, String>(0)?, row.get::<usize, i64>(1)?),
                    pass_hash: row.get(2)?,
                    totp: row.get(3)?,
                })
            },
        )
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Unpacks the userid into the username and session id.
pub fn unpack(userid: String) -> Result<(String, i64), DBError> {
    let split: Vec<&str> = userid.split(':').collect();
    Ok((
        split.get(0).map(|&username| username.into()).ok_or(DBError::InvalidUserID)?,
        split.get(1).and_then(|id| id.parse::<i64>().ok()).ok_or(DBError::InvalidUserID)?,
    ))
}

/// Returns password hash from userid, if userid is not valid returns [`None`].
pub async fn get_passhash(pool: &Pool, username: String, sessionid: i64) -> Result<Option<String>, DBError> {
    pool.conn(move |conn| {
        conn.query_row(
            "SELECT pass_hash FROM users WHERE username=?1 AND sessionid=?2",
            params![username, sessionid],
            |row| row.get(0),
        )
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get passhash from userid: {e}")))
}

/// Returns username and user admin status from userid.
/// If the userid is not valid returns [`None`].
pub async fn userinfo(pool: &Pool, userid: String) -> Result<Option<(String, bool)>, DBError> {
    let (username, sessionid) = unpack(userid)?;
    pool.conn(move |conn| {
        conn.query_row(
            "SELECT username, is_admin FROM users WHERE username=?1 AND sessionid=?2",
            params![username, sessionid],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to get user: {e}")))
}

/// Generates a new session id. Since the user must login again, the function doesn't return
/// the new userid. Returns error if the userid is not valid.
pub async fn change_sessionid(pool: &Pool, userid: String) -> Result<(), DBError> {
    let (username, old_sessionid) = unpack(userid)?;
    let new_sessionid: i64 = rand::random();
    pool.conn(move |conn| {
        conn.execute(
            "UPDATE users SET sessionid=?1 WHERE username=?2 AND sessionid=?3",
            params![new_sessionid, username, old_sessionid],
        )
    })
    .await
    .map_err(|e| DBError::ExecError(format!("Failed to change session id: {e}")))
    .and_then(|changes| if changes == 0 { Err(DBError::InvalidUserID) } else { Ok(()) })
}

/// Changes password's hash of the selected user
pub async fn change_passhash(pool: &Pool, username: String, new_pwdhash: String) -> Result<(), DBError> {
    pool.conn(|conn| conn.execute("UPDATE users SET pass_hash=?1 WHERE username=?2", [new_pwdhash, username]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to change user's passhash: {e}")))
        .and_then(|changes| if changes > 0 { Ok(()) } else { Err(DBError::UserNotFound) })
}

pub async fn change_totp(pool: &Pool, username: String, new_totp: String) -> Result<(), DBError> {
    pool.conn(|conn| conn.execute("UPDATE users SET totp=?1 WHERE username=?2", [new_totp, username]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to change user's TOTP: {e}")))
        .and_then(|changes| if changes > 0 { Ok(()) } else { Err(DBError::UserNotFound) })
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
    let (username, sessionid) = unpack(userid)?;
    let user = username.clone();
    pool.conn(move |conn| conn.execute("DELETE FROM users WHERE username=?1 AND sessionid=?2", params![username, sessionid]))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to delete user '{user}': {e}")))
        .and_then(|changes| if changes == 0 { Err(DBError::InvalidUserID) } else { Ok(()) })?;
    log::info!("Deleted user '{user}'");
    tokio::spawn(async move {
        if let Err(e) = super::delete_user_dir(&user).await {
            log::error!("Failed to delete user directory: {e}");
        }
        log::info!("Deleted user directory of '{user}'");
    });
    Ok(())
}
