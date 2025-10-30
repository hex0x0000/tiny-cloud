// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod auth;
pub mod error;
pub mod token;
pub mod utils;
use crate::{config, plugins};
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use auth::{USERS_TABLE, get_all_usernames};
use error::DBError;
use std::path::PathBuf;
use token::TOKEN_TABLE;
use tokio::fs;

fn tables() -> String {
    if config!(registration).is_some() {
        format!("BEGIN;{USERS_TABLE};{TOKEN_TABLE};COMMIT;")
    } else {
        format!("BEGIN;{USERS_TABLE};COMMIT;")
    }
}

async fn create_user_dir(user: &str) -> Result<(), DBError> {
    let mut path = PathBuf::from(config!(data_directory));
    path.push("users");
    path.push(user);
    for plugin in plugins::list() {
        path.push(plugin.name);
        fs::create_dir_all(&path)
            .await
            .map_err(|e| DBError::IOError(format!("Failed to create user directory: {e}")))?;
        path.pop();
    }
    Ok(())
}

async fn delete_user_dir(user: &str) -> Result<(), DBError> {
    let mut data_dir = PathBuf::from(config!(data_directory));
    data_dir.push("users");
    data_dir.push(user);
    fs::remove_dir_all(data_dir)
        .await
        .map_err(|e| DBError::IOError(format!("Failed to delete user directory: {e}")))?;
    Ok(())
}

/// Connects to sqlite database and returns a pool.
/// Sets it to Wal mode by default, which is better for concurrency.
pub async fn init() -> Result<Pool, DBError> {
    let mut data_path = PathBuf::from(config!(data_directory));

    // Create base data directory
    fs::create_dir_all(&data_path)
        .await
        .map_err(|e| DBError::IOError(format!("Failed to create data directory: {e}")))?;

    // Open/Create database
    data_path.push("auth.db");
    let pool = PoolBuilder::new()
        .journal_mode(JournalMode::Wal)
        .path(&data_path)
        .open()
        .await
        .map_err(|e| DBError::IOError(e.to_string()))?;
    data_path.pop();

    // Crates tables if they do not exist yet
    pool.conn(|conn| conn.execute_batch(&tables()))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to initialize tables: {e}")))?;

    // Crates directories for all of the users if they do not exist yet
    for user in get_all_usernames(&pool).await? {
        create_user_dir(&user).await?;
    }

    // Creates directory for plugin requests without a user
    data_path.push("unauth");
    for plugin in plugins::list() {
        data_path.push(plugin.name);
        fs::create_dir_all(&data_path)
            .await
            .map_err(|e| DBError::IOError(format!("Failed to create unauth directory: {e}")))?;
        data_path.pop();
    }

    Ok(pool)
}
