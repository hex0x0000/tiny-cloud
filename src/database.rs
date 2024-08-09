// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

pub mod auth;
pub mod error;
pub mod token;
pub mod utils;
use crate::{config, plugins};
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use auth::{get_all_usernames, USERS_TABLE};
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
        path.push(plugin);
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
        data_path.push(plugin);
        fs::create_dir_all(&data_path)
            .await
            .map_err(|e| DBError::IOError(format!("Failed to create unauth directory: {e}")))?;
        data_path.pop();
    }

    Ok(pool)
}
